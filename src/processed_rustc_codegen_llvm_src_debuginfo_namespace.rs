// SRC: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/namespace.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
// Namespace Handling.

use crate::rustc_codegen_ssa::debuginfo::type_names;
use crate::rustc_complete::def_id::DefId;
use crate::rustc_complete::ty::{self, Instance};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use super::utils::{DIB, debug_context};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */
use crate::common::CodegenCx;
use crate::llvm;
use crate::llvm::debuginfo::DIScope;

pub(crate) fn mangled_name_of_instance<'a, 'tcx>(
    cx: &CodegenCx<'a, 'tcx>,
    instance: Instance<'tcx>,
) -> ty::SymbolName<'tcx> {
    cx.tcx.symbol_name(instance)
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=15 | LINES=30 */

pub(crate) fn item_namespace<'ll>(cx: &CodegenCx<'ll, '_>, def_id: DefId) -> &'ll DIScope {
    if let Some(&scope) = debug_context(cx).namespace_map.borrow().get(&def_id) {
        return scope;
    }

    let def_key = cx.tcx.def_key(def_id);
    let parent_scope = def_key
        .parent
        .map(|parent| item_namespace(cx, DefId { krate: def_id.krate, index: parent }));

    let namespace_name_string = {
        let mut output = String::with_capacity(64);
        type_names::push_item_name(cx.tcx, def_id, false, &mut output);
        output
    };

    let scope = unsafe {
        llvm::LLVMDIBuilderCreateNameSpace(
            DIB(cx),
            parent_scope,
            namespace_name_string.as_ptr(),
            namespace_name_string.len(),
            llvm::FALSE, // ExportSymbols (only relevant for C++ anonymous namespaces)
        )
    };

    debug_context(cx).namespace_map.borrow_mut().insert(def_id, scope);
    scope
}