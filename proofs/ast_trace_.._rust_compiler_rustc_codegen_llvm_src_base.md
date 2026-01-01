# AST Trace: ../rust/compiler/rustc_codegen_llvm/src/base.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=34

```rust
// Codegen the MIR to the LLVM IR.
//
// Hopefully useful general knowledge about codegen:
//
// * There's no way to find out the [`Ty`] type of a [`Value`]. Doing so
//   would be "trying to get the eggs out of an omelette" (credit:
//   pcwalton). You can, instead, find out its [`llvm::Type`] by calling [`val_ty`],
//   but one [`llvm::Type`] corresponds to many [`Ty`]s; for instance, `tup(int, int,
//   int)` and `rec(x=int, y=int, z=int)` will have the same [`llvm::Type`].
//
// [`Ty`]: crate::rustc_middle::ty::Ty
// [`val_ty`]: crate::common::val_ty

use std::time::Instant;

use crate::rustc_codegen_ssa::ModuleCodegen;
use crate::rustc_codegen_ssa::base::maybe_create_entry_wrapper;
use crate::rustc_codegen_ssa::mono_item::MonoItemExt;
use crate::rustc_codegen_ssa::traits::*;
use crate::rustc_data_structures::small_c_str::SmallCStr;
use crate::rustc_complete::attrs::Linkage;
use crate::rustc_complete::dep_graph;
use crate::rustc_complete::middle::codegen_fn_attrs::CodegenFnAttrs;
use crate::rustc_complete::mir::mono::Visibility;
use crate::rustc_complete::ty::TyCtxt;
use crate::rustc_complete::config::DebugInfo;
use crate::rustc_complete::Symbol;
use crate::rustc_target::spec::SanitizerSet;

use super::ModuleLlvm;
use crate::builder::Builder;
use crate::context::CodegenCx;
use crate::value::Value;
use crate::{attributes, llvm};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=5

```rust
pub(crate) struct ValueIter<'ll> {
    cur: Option<&'ll Value>,
    step: unsafe extern "C" fn(&'ll Value) -> Option<&'ll Value>,
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=next | COMPLEXITY=13 | LINES=12

```rust
impl<'ll> Iterator for ValueIter<'ll> {
    type Item = &'ll Value;

    fn next(&mut self) -> Option<&'ll Value> {
        let old = self.cur;
        if let Some(old) = old {
            self.cur = unsafe { (self.step)(old) };
        }
        old
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
pub(crate) fn iter_globals(llmod: &llvm::Module) -> ValueIter<'_> {
    unsafe { ValueIter { cur: llvm::LLVMGetFirstGlobal(llmod), step: llvm::LLVMGetNextGlobal } }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=module_codegen | COMPLEXITY=45 | LINES=90

```rust
pub(crate) fn compile_codegen_unit(
    tcx: TyCtxt<'_>,
    cgu_name: Symbol,
) -> (ModuleCodegen<ModuleLlvm>, u64) {
    let start_time = Instant::now();

    let dep_node = tcx.codegen_unit(cgu_name).codegen_dep_node(tcx);
    let (module, _) = tcx.dep_graph.with_task(
        dep_node,
        tcx,
        cgu_name,
        module_codegen,
        Some(dep_graph::hash_result),
    );
    let time_to_codegen = start_time.elapsed();

    // We assume that the cost to run LLVM on a CGU is proportional to
    // the time we needed for codegenning it.
    let cost = time_to_codegen.as_nanos() as u64;

    fn module_codegen(tcx: TyCtxt<'_>, cgu_name: Symbol) -> ModuleCodegen<ModuleLlvm> {
        let cgu = tcx.codegen_unit(cgu_name);
        let _prof_timer =
            tcx.prof.generic_activity_with_arg_recorder("codegen_module", |recorder| {
                recorder.record_arg(cgu_name.to_string());
                recorder.record_arg(cgu.size_estimate().to_string());
            });
        // Instantiate monomorphizations without filling out definitions yet...
        let llvm_module = ModuleLlvm::new(tcx, cgu_name.as_str());
        {
            let mut cx = CodegenCx::new(tcx, cgu, &llvm_module);
            let mono_items = cx.codegen_unit.items_in_deterministic_order(cx.tcx);
            for &(mono_item, data) in &mono_items {
                mono_item.predefine::<Builder<'_, '_, '_>>(
                    &mut cx,
                    cgu_name.as_str(),
                    data.linkage,
                    data.visibility,
                );
            }

            // ... and now that we have everything pre-defined, fill out those definitions.
            for &(mono_item, item_data) in &mono_items {
                mono_item.define::<Builder<'_, '_, '_>>(&mut cx, cgu_name.as_str(), item_data);
            }

            // If this codegen unit contains the main function, also create the
            // wrapper here
            if let Some(entry) =
                maybe_create_entry_wrapper::<Builder<'_, '_, '_>>(&cx, cx.codegen_unit)
            {
                let attrs = attributes::sanitize_attrs(&cx, SanitizerSet::empty());
                attributes::apply_to_llfn(entry, llvm::AttributePlace::Function, &attrs);
            }

            // Finalize code coverage by injecting the coverage map. Note, the coverage map will
            // also be added to the `llvm.compiler.used` variable, created next.
            if cx.sess().instrument_coverage() {
                cx.coverageinfo_finalize();
            }

            // Create the llvm.used and llvm.compiler.used variables.
            if !cx.used_statics.is_empty() {
                cx.create_used_variable_impl(c"llvm.used", &cx.used_statics);
            }
            if !cx.compiler_used_statics.is_empty() {
                cx.create_used_variable_impl(c"llvm.compiler.used", &cx.compiler_used_statics);
            }

            // Run replace-all-uses-with for statics that need it. This must
            // happen after the llvm.used variables are created.
            for &(old_g, new_g) in cx.statics_to_rauw().borrow().iter() {
                unsafe {
                    llvm::LLVMReplaceAllUsesWith(old_g, new_g);
                    llvm::LLVMDeleteGlobal(old_g);
                }
            }

            // Finalize debuginfo
            if cx.sess().opts.debuginfo != DebugInfo::None {
                cx.debuginfo_finalize();
            }
        }

        ModuleCodegen::new_regular(cgu_name.to_string(), llvm_module)
    }

    (module, cost)
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=6

```rust
pub(crate) fn set_link_section(llval: &Value, attrs: &CodegenFnAttrs) {
    let Some(sect) = attrs.link_section else { return };
    let buf = SmallCStr::new(sect.as_str());
    llvm::set_section(llval, &buf);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=14

```rust
pub(crate) fn linkage_to_llvm(linkage: Linkage) -> llvm::Linkage {
    match linkage {
        Linkage::External => llvm::Linkage::ExternalLinkage,
        Linkage::AvailableExternally => llvm::Linkage::AvailableExternallyLinkage,
        Linkage::LinkOnceAny => llvm::Linkage::LinkOnceAnyLinkage,
        Linkage::LinkOnceODR => llvm::Linkage::LinkOnceODRLinkage,
        Linkage::WeakAny => llvm::Linkage::WeakAnyLinkage,
        Linkage::WeakODR => llvm::Linkage::WeakODRLinkage,
        Linkage::Internal => llvm::Linkage::InternalLinkage,
        Linkage::ExternalWeak => llvm::Linkage::ExternalWeakLinkage,
        Linkage::Common => llvm::Linkage::CommonLinkage,
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=8

```rust
pub(crate) fn visibility_to_llvm(linkage: Visibility) -> llvm::Visibility {
    match linkage {
        Visibility::Default => llvm::Visibility::Default,
        Visibility::Hidden => llvm::Visibility::Hidden,
        Visibility::Protected => llvm::Visibility::Protected,
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=18 | LINES=9

```rust
pub(crate) fn set_variable_sanitizer_attrs(llval: &Value, attrs: &CodegenFnAttrs) {
    if attrs.no_sanitize.contains(SanitizerSet::ADDRESS) {
        unsafe { llvm::LLVMRustSetNoSanitizeAddress(llval) };
    }
    if attrs.no_sanitize.contains(SanitizerSet::HWADDRESS) {
        unsafe { llvm::LLVMRustSetNoSanitizeHWAddress(llval) };
    }
}
```

---
*Generated by AST tracing system*
