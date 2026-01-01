# AST Trace: ../rust/compiler/rustc_codegen_ssa/src/back/symbol_export.rs

Generated 29 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use std::collections::hash_map::Entry::*;

use crate::rustc_abi::{CanonAbi, X86Call};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::rustc_complete::expand::allocator::{ALLOCATOR_METHODS, NO_ALLOC_SHIM_IS_UNSTABLE, global_fn_name};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::rustc_data_structures::unord::UnordMap;
use crate::rustc_complete::def::DefKind;
use crate::rustc_complete::def_id::{CrateNum, DefId, DefIdMap, LOCAL_CRATE, LocalDefId};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use crate::rustc_complete::bug;
use crate::rustc_complete::middle::codegen_fn_attrs::CodegenFnAttrFlags;
use crate::rustc_complete::middle::exported_symbols::{
    ExportedSymbol, SymbolExportInfo, SymbolExportKind, SymbolExportLevel,
};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::rustc_complete::query::LocalCrate;
use crate::rustc_complete::ty::{self, GenericArgKind, GenericArgsRef, Instance, SymbolName, Ty, TyCtxt};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::rustc_complete::util::Providers;
use crate::rustc_complete::config::{CrateType, OomStrategy};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=threshold | COMPLEXITY=2 | LINES=9

```rust
use rustc_symbol_mangling::mangle_internal_symbol;
use crate::rustc_target::spec::TlsModel;
use tracing::debug;

use crate::back::symbol_export;

fn threshold(tcx: TyCtxt<'_>) -> SymbolExportLevel {
    crates_export_threshold(tcx.crate_types())
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=crate_export_threshold | COMPLEXITY=7 | LINES=9

```rust
fn crate_export_threshold(crate_type: CrateType) -> SymbolExportLevel {
    match crate_type {
        CrateType::Executable | CrateType::Staticlib | CrateType::ProcMacro | CrateType::Cdylib => {
            SymbolExportLevel::C
        }
        CrateType::Rlib | CrateType::Dylib | CrateType::Sdylib => SymbolExportLevel::Rust,
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=crates_export_threshold | COMPLEXITY=6 | LINES=11

```rust
pub fn crates_export_threshold(crate_types: &[CrateType]) -> SymbolExportLevel {
    if crate_types
        .iter()
        .any(|&crate_type| crate_export_threshold(crate_type) == SymbolExportLevel::Rust)
    {
        SymbolExportLevel::Rust
    } else {
        SymbolExportLevel::C
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=reachable_non_generics_provider | COMPLEXITY=67 | LINES=110

```rust
fn reachable_non_generics_provider(tcx: TyCtxt<'_>, _: LocalCrate) -> DefIdMap<SymbolExportInfo> {
    if !tcx.sess.opts.output_types.should_codegen() && !tcx.is_sdylib_interface_build() {
        return Default::default();
    }

    // Check to see if this crate is a "special runtime crate". These
    // crates, implementation details of the standard library, typically
    // have a bunch of `pub extern` and `#[unsafe(no_mangle)]` functions as the
    // ABI between them. We don't want their symbols to have a `C`
    // export level, however, as they're just implementation details.
    // Down below we'll hardwire all of the symbols to the `Rust` export
    // level instead.
    let special_runtime_crate =
        tcx.is_panic_runtime(LOCAL_CRATE) || tcx.is_compiler_builtins(LOCAL_CRATE);

    let mut reachable_non_generics: DefIdMap<_> = tcx
        .reachable_set(())
        .items()
        .filter_map(|&def_id| {
            // We want to ignore some FFI functions that are not exposed from
            // this crate. Reachable FFI functions can be lumped into two
            // categories:
            //
            // 1. Those that are included statically via a static library
            // 2. Those included otherwise (e.g., dynamically or via a framework)
            //
            // Although our LLVM module is not literally emitting code for the
            // statically included symbols, it's an export of our library which
            // needs to be passed on to the linker and encoded in the metadata.
            //
            // As a result, if this id is an FFI item (foreign item) then we only
            // let it through if it's included statically.
            if let Some(parent_id) = tcx.opt_local_parent(def_id)
                && let DefKind::ForeignMod = tcx.def_kind(parent_id)
            {
                let library = tcx.native_library(def_id)?;
                return library.kind.is_statically_included().then_some(def_id);
            }

            // Only consider nodes that actually have exported symbols.
            match tcx.def_kind(def_id) {
                DefKind::Fn | DefKind::Static { .. } => {}
                DefKind::AssocFn if tcx.impl_of_assoc(def_id.to_def_id()).is_some() => {}
                _ => return None,
            };

            let generics = tcx.generics_of(def_id);
            if generics.requires_monomorphization(tcx) {
                return None;
            }

            if Instance::mono(tcx, def_id.into()).def.requires_inline(tcx) {
                return None;
            }

            if tcx.cross_crate_inlinable(def_id) { None } else { Some(def_id) }
        })
        .map(|def_id| {
            // We won't link right if this symbol is stripped during LTO.
            let name = tcx.symbol_name(Instance::mono(tcx, def_id.to_def_id())).name;
            let used = name == "rust_eh_personality";

            let export_level = if special_runtime_crate {
                SymbolExportLevel::Rust
            } else {
                symbol_export_level(tcx, def_id.to_def_id())
            };
            let codegen_attrs = tcx.codegen_fn_attrs(def_id.to_def_id());
            debug!(
                "EXPORTED SYMBOL (local): {} ({:?})",
                tcx.symbol_name(Instance::mono(tcx, def_id.to_def_id())),
                export_level
            );
            let info = SymbolExportInfo {
                level: export_level,
                kind: if tcx.is_static(def_id.to_def_id()) {
                    if codegen_attrs.flags.contains(CodegenFnAttrFlags::THREAD_LOCAL) {
                        SymbolExportKind::Tls
                    } else {
                        SymbolExportKind::Data
                    }
                } else {
                    SymbolExportKind::Text
                },
                used: codegen_attrs.flags.contains(CodegenFnAttrFlags::USED_COMPILER)
                    || codegen_attrs.flags.contains(CodegenFnAttrFlags::USED_LINKER)
                    || used,
                rustc_std_internal_symbol: codegen_attrs
                    .flags
                    .contains(CodegenFnAttrFlags::RUSTC_STD_INTERNAL_SYMBOL),
            };
            (def_id.to_def_id(), info)
        })
        .into();

    if let Some(id) = tcx.proc_macro_decls_static(()) {
        reachable_non_generics.insert(
            id.to_def_id(),
            SymbolExportInfo {
                level: SymbolExportLevel::C,
                kind: SymbolExportKind::Data,
                used: false,
                rustc_std_internal_symbol: false,
            },
        );
    }

    reachable_non_generics
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=is_reachable_non_generic_provider_local | COMPLEXITY=6 | LINES=10

```rust
fn is_reachable_non_generic_provider_local(tcx: TyCtxt<'_>, def_id: LocalDefId) -> bool {
    let export_threshold = threshold(tcx);

    if let Some(&info) = tcx.reachable_non_generics(LOCAL_CRATE).get(&def_id.to_def_id()) {
        info.level.is_below_threshold(export_threshold)
    } else {
        false
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=is_reachable_non_generic_provider_extern | COMPLEXITY=2 | LINES=4

```rust
fn is_reachable_non_generic_provider_extern(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    tcx.reachable_non_generics(def_id.krate).contains_key(&def_id)
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=exported_non_generic_symbols_provider_local | COMPLEXITY=19 | LINES=55

```rust
fn exported_non_generic_symbols_provider_local<'tcx>(
    tcx: TyCtxt<'tcx>,
    _: LocalCrate,
) -> &'tcx [(ExportedSymbol<'tcx>, SymbolExportInfo)] {
    if !tcx.sess.opts.output_types.should_codegen() && !tcx.is_sdylib_interface_build() {
        return &[];
    }

    // FIXME: Sorting this is unnecessary since we are sorting later anyway.
    //        Can we skip the later sorting?
    let sorted = tcx.with_stable_hashing_context(|hcx| {
        tcx.reachable_non_generics(LOCAL_CRATE).to_sorted(&hcx, true)
    });

    let mut symbols: Vec<_> =
        sorted.iter().map(|&(&def_id, &info)| (ExportedSymbol::NonGeneric(def_id), info)).collect();

    // Export TLS shims
    if !tcx.sess.target.dll_tls_export {
        symbols.extend(sorted.iter().filter_map(|&(&def_id, &info)| {
            tcx.needs_thread_local_shim(def_id).then(|| {
                (
                    ExportedSymbol::ThreadLocalShim(def_id),
                    SymbolExportInfo {
                        level: info.level,
                        kind: SymbolExportKind::Text,
                        used: info.used,
                        rustc_std_internal_symbol: info.rustc_std_internal_symbol,
                    },
                )
            })
        }))
    }

    if tcx.entry_fn(()).is_some() {
        let exported_symbol =
            ExportedSymbol::NoDefId(SymbolName::new(tcx, tcx.sess.target.entry_name.as_ref()));

        symbols.push((
            exported_symbol,
            SymbolExportInfo {
                level: SymbolExportLevel::C,
                kind: SymbolExportKind::Text,
                used: false,
                rustc_std_internal_symbol: false,
            },
        ));
    }

    // Sort so we get a stable incr. comp. hash.
    symbols.sort_by_cached_key(|s| s.0.symbol_name_for_local_instance(tcx));

    tcx.arena.alloc_from_iter(symbols)
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=exported_generic_symbols_provider_local | COMPLEXITY=85 | LINES=167

```rust
fn exported_generic_symbols_provider_local<'tcx>(
    tcx: TyCtxt<'tcx>,
    _: LocalCrate,
) -> &'tcx [(ExportedSymbol<'tcx>, SymbolExportInfo)] {
    if !tcx.sess.opts.output_types.should_codegen() && !tcx.is_sdylib_interface_build() {
        return &[];
    }

    let mut symbols: Vec<_> = vec![];

    if tcx.local_crate_exports_generics() {
        use crate::rustc_complete::attrs::Linkage;
        use crate::rustc_complete::mir::mono::{MonoItem, Visibility};
        use crate::rustc_complete::ty::InstanceKind;

        // Normally, we require that shared monomorphizations are not hidden,
        // because if we want to re-use a monomorphization from a Rust dylib, it
        // needs to be exported.
        // However, on platforms that don't allow for Rust dylibs, having
        // external linkage is enough for monomorphization to be linked to.
        let need_visibility = tcx.sess.target.dynamic_linking && !tcx.sess.target.only_cdylib;

        let cgus = tcx.collect_and_partition_mono_items(()).codegen_units;

        // Do not export symbols that cannot be instantiated by downstream crates.
        let reachable_set = tcx.reachable_set(());
        let is_local_to_current_crate = |ty: Ty<'_>| {
            let no_refs = ty.peel_refs();
            let root_def_id = match no_refs.kind() {
                ty::Closure(closure, _) => *closure,
                ty::FnDef(def_id, _) => *def_id,
                ty::Coroutine(def_id, _) => *def_id,
                ty::CoroutineClosure(def_id, _) => *def_id,
                ty::CoroutineWitness(def_id, _) => *def_id,
                _ => return false,
            };
            let Some(root_def_id) = root_def_id.as_local() else {
                return false;
            };

            let is_local = !reachable_set.contains(&root_def_id);
            is_local
        };

        let is_instantiable_downstream =
            |did: Option<DefId>, generic_args: GenericArgsRef<'tcx>| {
                generic_args
                    .types()
                    .chain(did.into_iter().map(move |did| tcx.type_of(did).skip_binder()))
                    .all(move |arg| {
                        arg.walk().all(|ty| {
                            ty.as_type().map_or(true, |ty| !is_local_to_current_crate(ty))
                        })
                    })
            };

        // The symbols created in this loop are sorted below it
        #[allow(rustc::potential_query_instability)]
        for (mono_item, data) in cgus.iter().flat_map(|cgu| cgu.items().iter()) {
            if data.linkage != Linkage::External {
                // We can only re-use things with external linkage, otherwise
                // we'll get a linker error
                continue;
            }

            if need_visibility && data.visibility == Visibility::Hidden {
                // If we potentially share things from Rust dylibs, they must
                // not be hidden
                continue;
            }

            if !tcx.sess.opts.share_generics() {
                if tcx.codegen_fn_attrs(mono_item.def_id()).inline
                    == crate::rustc_hir::attrs::InlineAttr::Never
                {
                    // this is OK, we explicitly allow sharing inline(never) across crates even
                    // without share-generics.
                } else {
                    continue;
                }
            }

            // Note: These all set rustc_std_internal_symbol to false as generic functions must not
            // be marked with this attribute and we are only handling generic functions here.
            match *mono_item {
                MonoItem::Fn(Instance { def: InstanceKind::Item(def), args }) => {
                    let has_generics = args.non_erasable_generics().next().is_some();

                    let should_export =
                        has_generics && is_instantiable_downstream(Some(def), &args);

                    if should_export {
                        let symbol = ExportedSymbol::Generic(def, args);
                        symbols.push((
                            symbol,
                            SymbolExportInfo {
                                level: SymbolExportLevel::Rust,
                                kind: SymbolExportKind::Text,
                                used: false,
                                rustc_std_internal_symbol: false,
                            },
                        ));
                    }
                }
                MonoItem::Fn(Instance { def: InstanceKind::DropGlue(_, Some(ty)), args }) => {
                    // A little sanity-check
                    assert_eq!(args.non_erasable_generics().next(), Some(GenericArgKind::Type(ty)));

                    // Drop glue did is always going to be non-local outside of libcore, thus we don't need to check it's locality (which includes invoking `type_of` query).
                    let should_export = match ty.kind() {
                        ty::Adt(_, args) => is_instantiable_downstream(None, args),
                        ty::Closure(_, args) => is_instantiable_downstream(None, args),
                        _ => true,
                    };

                    if should_export {
                        symbols.push((
                            ExportedSymbol::DropGlue(ty),
                            SymbolExportInfo {
                                level: SymbolExportLevel::Rust,
                                kind: SymbolExportKind::Text,
                                used: false,
                                rustc_std_internal_symbol: false,
                            },
                        ));
                    }
                }
                MonoItem::Fn(Instance {
                    def: InstanceKind::AsyncDropGlueCtorShim(_, ty),
                    args,
                }) => {
                    // A little sanity-check
                    assert_eq!(args.non_erasable_generics().next(), Some(GenericArgKind::Type(ty)));
                    symbols.push((
                        ExportedSymbol::AsyncDropGlueCtorShim(ty),
                        SymbolExportInfo {
                            level: SymbolExportLevel::Rust,
                            kind: SymbolExportKind::Text,
                            used: false,
                            rustc_std_internal_symbol: false,
                        },
                    ));
                }
                MonoItem::Fn(Instance { def: InstanceKind::AsyncDropGlue(def, ty), args: _ }) => {
                    symbols.push((
                        ExportedSymbol::AsyncDropGlue(def, ty),
                        SymbolExportInfo {
                            level: SymbolExportLevel::Rust,
                            kind: SymbolExportKind::Text,
                            used: false,
                            rustc_std_internal_symbol: false,
                        },
                    ));
                }
                _ => {
                    // Any other symbols don't qualify for sharing
                }
            }
        }
    }

    // Sort so we get a stable incr. comp. hash.
    symbols.sort_by_cached_key(|s| s.0.symbol_name_for_local_instance(tcx));

    tcx.arena.alloc_from_iter(symbols)
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=upstream_monomorphizations_provider | COMPLEXITY=36 | LINES=58

```rust
fn upstream_monomorphizations_provider(
    tcx: TyCtxt<'_>,
    (): (),
) -> DefIdMap<UnordMap<GenericArgsRef<'_>, CrateNum>> {
    let cnums = tcx.crates(());

    let mut instances: DefIdMap<UnordMap<_, _>> = Default::default();

    let drop_in_place_fn_def_id = tcx.lang_items().drop_in_place_fn();
    let async_drop_in_place_fn_def_id = tcx.lang_items().async_drop_in_place_fn();

    for &cnum in cnums.iter() {
        for (exported_symbol, _) in tcx.exported_generic_symbols(cnum).iter() {
            let (def_id, args) = match *exported_symbol {
                ExportedSymbol::Generic(def_id, args) => (def_id, args),
                ExportedSymbol::DropGlue(ty) => {
                    if let Some(drop_in_place_fn_def_id) = drop_in_place_fn_def_id {
                        (drop_in_place_fn_def_id, tcx.mk_args(&[ty.into()]))
                    } else {
                        // `drop_in_place` in place does not exist, don't try
                        // to use it.
                        continue;
                    }
                }
                ExportedSymbol::AsyncDropGlueCtorShim(ty) => {
                    if let Some(async_drop_in_place_fn_def_id) = async_drop_in_place_fn_def_id {
                        (async_drop_in_place_fn_def_id, tcx.mk_args(&[ty.into()]))
                    } else {
                        continue;
                    }
                }
                ExportedSymbol::AsyncDropGlue(def_id, ty) => (def_id, tcx.mk_args(&[ty.into()])),
                ExportedSymbol::NonGeneric(..)
                | ExportedSymbol::ThreadLocalShim(..)
                | ExportedSymbol::NoDefId(..) => unreachable!("{exported_symbol:?}"),
            };

            let args_map = instances.entry(def_id).or_default();

            match args_map.entry(args) {
                Occupied(mut e) => {
                    // If there are multiple monomorphizations available,
                    // we select one deterministically.
                    let other_cnum = *e.get();
                    if tcx.stable_crate_id(other_cnum) > tcx.stable_crate_id(cnum) {
                        e.insert(cnum);
                    }
                }
                Vacant(e) => {
                    e.insert(cnum);
                }
            }
        }
    }

    instances
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=upstream_monomorphizations_for_provider | COMPLEXITY=2 | LINES=8

```rust
fn upstream_monomorphizations_for_provider(
    tcx: TyCtxt<'_>,
    def_id: DefId,
) -> Option<&UnordMap<GenericArgsRef<'_>, CrateNum>> {
    assert!(!def_id.is_local());
    tcx.upstream_monomorphizations(()).get(&def_id)
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=upstream_drop_glue_for_provider | COMPLEXITY=2 | LINES=8

```rust
fn upstream_drop_glue_for_provider<'tcx>(
    tcx: TyCtxt<'tcx>,
    args: GenericArgsRef<'tcx>,
) -> Option<CrateNum> {
    let def_id = tcx.lang_items().drop_in_place_fn()?;
    tcx.upstream_monomorphizations_for(def_id)?.get(&args).cloned()
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=upstream_async_drop_glue_for_provider | COMPLEXITY=2 | LINES=8

```rust
fn upstream_async_drop_glue_for_provider<'tcx>(
    tcx: TyCtxt<'tcx>,
    args: GenericArgsRef<'tcx>,
) -> Option<CrateNum> {
    let def_id = tcx.lang_items().async_drop_in_place_fn()?;
    tcx.upstream_monomorphizations_for(def_id)?.get(&args).cloned()
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=is_unreachable_local_definition_provider | COMPLEXITY=2 | LINES=4

```rust
fn is_unreachable_local_definition_provider(tcx: TyCtxt<'_>, def_id: LocalDefId) -> bool {
    !tcx.reachable_set(()).contains(&def_id)
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=9 | LINES=15

```rust
pub(crate) fn provide(providers: &mut Providers) {
    providers.reachable_non_generics = reachable_non_generics_provider;
    providers.is_reachable_non_generic = is_reachable_non_generic_provider_local;
    providers.exported_non_generic_symbols = exported_non_generic_symbols_provider_local;
    providers.exported_generic_symbols = exported_generic_symbols_provider_local;
    providers.upstream_monomorphizations = upstream_monomorphizations_provider;
    providers.is_unreachable_local_definition = is_unreachable_local_definition_provider;
    providers.upstream_drop_glue_for = upstream_drop_glue_for_provider;
    providers.upstream_async_drop_glue_for = upstream_async_drop_glue_for_provider;
    providers.wasm_import_module_map = wasm_import_module_map;
    providers.extern_queries.is_reachable_non_generic = is_reachable_non_generic_provider_extern;
    providers.extern_queries.upstream_monomorphizations_for =
        upstream_monomorphizations_for_provider;
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=25

```rust
pub(crate) fn allocator_shim_symbols(
    tcx: TyCtxt<'_>,
) -> impl Iterator<Item = (String, SymbolExportKind)> {
    ALLOCATOR_METHODS
        .iter()
        .map(move |method| mangle_internal_symbol(tcx, global_fn_name(method.name).as_str()))
        .chain([
            mangle_internal_symbol(tcx, "__rust_alloc_error_handler"),
            mangle_internal_symbol(tcx, OomStrategy::SYMBOL),
            mangle_internal_symbol(tcx, NO_ALLOC_SHIM_IS_UNSTABLE),
        ])
        .map(move |symbol_name| {
            let exported_symbol = ExportedSymbol::NoDefId(SymbolName::new(tcx, &symbol_name));

            (
                symbol_export::exporting_symbol_name_for_instance_in_crate(
                    tcx,
                    exported_symbol,
                    LOCAL_CRATE,
                ),
                SymbolExportKind::Text,
            )
        })
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=symbol_export_level | COMPLEXITY=19 | LINES=26

```rust
fn symbol_export_level(tcx: TyCtxt<'_>, sym_def_id: DefId) -> SymbolExportLevel {
    // We export anything that's not mangled at the "C" layer as it probably has
    // to do with ABI concerns. We do not, however, apply such treatment to
    // special symbols in the standard library for various plumbing between
    // core/std/allocators/etc. For example symbols used to hook up allocation
    // are not considered for export
    let codegen_fn_attrs = tcx.codegen_fn_attrs(sym_def_id);
    let is_extern = codegen_fn_attrs.contains_extern_indicator();
    let std_internal =
        codegen_fn_attrs.flags.contains(CodegenFnAttrFlags::RUSTC_STD_INTERNAL_SYMBOL);

    if is_extern && !std_internal {
        let target = &tcx.sess.target.llvm_target;
        // WebAssembly cannot export data symbols, so reduce their export level
        if target.contains("emscripten") {
            if let DefKind::Static { .. } = tcx.def_kind(sym_def_id) {
                return SymbolExportLevel::Rust;
            }
        }

        SymbolExportLevel::C
    } else {
        SymbolExportLevel::Rust
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=19 | LINES=62

```rust
/// This is the symbol name of the given instance instantiated in a specific crate.
pub(crate) fn symbol_name_for_instance_in_crate<'tcx>(
    tcx: TyCtxt<'tcx>,
    symbol: ExportedSymbol<'tcx>,
    instantiating_crate: CrateNum,
) -> String {
    // If this is something instantiated in the local crate then we might
    // already have cached the name as a query result.
    if instantiating_crate == LOCAL_CRATE {
        return symbol.symbol_name_for_local_instance(tcx).to_string();
    }

    // This is something instantiated in an upstream crate, so we have to use
    // the slower (because uncached) version of computing the symbol name.
    match symbol {
        ExportedSymbol::NonGeneric(def_id) => {
            rustc_symbol_mangling::symbol_name_for_instance_in_crate(
                tcx,
                Instance::mono(tcx, def_id),
                instantiating_crate,
            )
        }
        ExportedSymbol::Generic(def_id, args) => {
            rustc_symbol_mangling::symbol_name_for_instance_in_crate(
                tcx,
                Instance::new_raw(def_id, args),
                instantiating_crate,
            )
        }
        ExportedSymbol::ThreadLocalShim(def_id) => {
            rustc_symbol_mangling::symbol_name_for_instance_in_crate(
                tcx,
                ty::Instance {
                    def: ty::InstanceKind::ThreadLocalShim(def_id),
                    args: ty::GenericArgs::empty(),
                },
                instantiating_crate,
            )
        }
        ExportedSymbol::DropGlue(ty) => rustc_symbol_mangling::symbol_name_for_instance_in_crate(
            tcx,
            Instance::resolve_drop_in_place(tcx, ty),
            instantiating_crate,
        ),
        ExportedSymbol::AsyncDropGlueCtorShim(ty) => {
            rustc_symbol_mangling::symbol_name_for_instance_in_crate(
                tcx,
                Instance::resolve_async_drop_in_place(tcx, ty),
                instantiating_crate,
            )
        }
        ExportedSymbol::AsyncDropGlue(def_id, ty) => {
            rustc_symbol_mangling::symbol_name_for_instance_in_crate(
                tcx,
                Instance::resolve_async_drop_in_place_poll(tcx, def_id, ty),
                instantiating_crate,
            )
        }
        ExportedSymbol::NoDefId(symbol_name) => symbol_name.to_string(),
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=calling_convention_for_symbol | COMPLEXITY=14 | LINES=37

```rust
fn calling_convention_for_symbol<'tcx>(
    tcx: TyCtxt<'tcx>,
    symbol: ExportedSymbol<'tcx>,
) -> (CanonAbi, &'tcx [crate::rustc_target::callconv::ArgAbi<'tcx, Ty<'tcx>>]) {
    let instance = match symbol {
        ExportedSymbol::NonGeneric(def_id) | ExportedSymbol::Generic(def_id, _)
            if tcx.is_static(def_id) =>
        {
            None
        }
        ExportedSymbol::NonGeneric(def_id) => Some(Instance::mono(tcx, def_id)),
        ExportedSymbol::Generic(def_id, args) => Some(Instance::new_raw(def_id, args)),
        // DropGlue always use the Rust calling convention and thus follow the target's default
        // symbol decoration scheme.
        ExportedSymbol::DropGlue(..) => None,
        // AsyncDropGlueCtorShim always use the Rust calling convention and thus follow the
        // target's default symbol decoration scheme.
        ExportedSymbol::AsyncDropGlueCtorShim(..) => None,
        ExportedSymbol::AsyncDropGlue(..) => None,
        // NoDefId always follow the target's default symbol decoration scheme.
        ExportedSymbol::NoDefId(..) => None,
        // ThreadLocalShim always follow the target's default symbol decoration scheme.
        ExportedSymbol::ThreadLocalShim(..) => None,
    };

    instance
        .map(|i| {
            tcx.fn_abi_of_instance(
                ty::TypingEnv::fully_monomorphized().as_query_input((i, ty::List::empty())),
            )
            .unwrap_or_else(|_| bug!("fn_abi_of_instance({i:?}) failed"))
        })
        .map(|fnabi| (fnabi.conv, &fnabi.args[..]))
        // FIXME(workingjubilee): why don't we know the convention here?
        .unwrap_or((CanonAbi::Rust, &[]))
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=32 | LINES=56

```rust
/// This is the symbol name of the given instance as seen by the linker.
///
/// On 32-bit Windows symbols are decorated according to their calling conventions.
pub(crate) fn linking_symbol_name_for_instance_in_crate<'tcx>(
    tcx: TyCtxt<'tcx>,
    symbol: ExportedSymbol<'tcx>,
    export_kind: SymbolExportKind,
    instantiating_crate: CrateNum,
) -> String {
    let mut undecorated = symbol_name_for_instance_in_crate(tcx, symbol, instantiating_crate);

    // thread local will not be a function call,
    // so it is safe to return before windows symbol decoration check.
    if let Some(name) = maybe_emutls_symbol_name(tcx, symbol, &undecorated) {
        return name;
    }

    let target = &tcx.sess.target;
    if !target.is_like_windows {
        // Mach-O has a global "_" suffix and `object` crate will handle it.
        // ELF does not have any symbol decorations.
        return undecorated;
    }

    let prefix = match &target.arch[..] {
        "x86" => Some('_'),
        "x86_64" => None,
        // Only functions are decorated for arm64ec.
        "arm64ec" if export_kind == SymbolExportKind::Text => Some('#'),
        // Only x86/64 and arm64ec use symbol decorations.
        _ => return undecorated,
    };

    let (callconv, args) = calling_convention_for_symbol(tcx, symbol);

    // Decorate symbols with prefixes, suffixes and total number of bytes of arguments.
    // Reference: https://docs.microsoft.com/en-us/cpp/build/reference/decorated-names?view=msvc-170
    let (prefix, suffix) = match callconv {
        CanonAbi::X86(X86Call::Fastcall) => ("@", "@"),
        CanonAbi::X86(X86Call::Stdcall) => ("_", "@"),
        CanonAbi::X86(X86Call::Vectorcall) => ("", "@@"),
        _ => {
            if let Some(prefix) = prefix {
                undecorated.insert(0, prefix);
            }
            return undecorated;
        }
    };

    let args_in_bytes: u64 = args
        .iter()
        .map(|abi| abi.layout.size.bytes().next_multiple_of(target.pointer_width as u64 / 8))
        .sum();
    format!("{prefix}{undecorated}{suffix}{args_in_bytes}")
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
pub(crate) fn exporting_symbol_name_for_instance_in_crate<'tcx>(
    tcx: TyCtxt<'tcx>,
    symbol: ExportedSymbol<'tcx>,
    cnum: CrateNum,
) -> String {
    let undecorated = symbol_name_for_instance_in_crate(tcx, symbol, cnum);
    maybe_emutls_symbol_name(tcx, symbol, &undecorated).unwrap_or(undecorated)
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=23

```rust
/// On amdhsa, `gpu-kernel` functions have an associated metadata object with a `.kd` suffix.
/// Add it to the symbols list for all kernel functions, so that it is exported in the linked
/// object.
pub(crate) fn extend_exported_symbols<'tcx>(
    symbols: &mut Vec<(String, SymbolExportKind)>,
    tcx: TyCtxt<'tcx>,
    symbol: ExportedSymbol<'tcx>,
    instantiating_crate: CrateNum,
) {
    let (callconv, _) = calling_convention_for_symbol(tcx, symbol);

    if callconv != CanonAbi::GpuKernel || tcx.sess.target.os != "amdhsa" {
        return;
    }

    let undecorated = symbol_name_for_instance_in_crate(tcx, symbol, instantiating_crate);

    // Add the symbol for the kernel descriptor (with .kd suffix)
    // Per https://llvm.org/docs/AMDGPUUsage.html#symbols these will always be `STT_OBJECT` so
    // export as data.
    symbols.push((format!("{undecorated}.kd"), SymbolExportKind::Data));
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=maybe_emutls_symbol_name | COMPLEXITY=11 | LINES=17

```rust
fn maybe_emutls_symbol_name<'tcx>(
    tcx: TyCtxt<'tcx>,
    symbol: ExportedSymbol<'tcx>,
    undecorated: &str,
) -> Option<String> {
    if matches!(tcx.sess.tls_model(), TlsModel::Emulated)
        && let ExportedSymbol::NonGeneric(def_id) = symbol
        && tcx.is_thread_local_static(def_id)
    {
        // When using emutls, LLVM will add the `__emutls_v.` prefix to thread local symbols,
        // and exported symbol name need to match this.
        Some(format!("__emutls_v.{undecorated}"))
    } else {
        None
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=wasm_import_module_map | COMPLEXITY=10 | LINES=24

```rust
fn wasm_import_module_map(tcx: TyCtxt<'_>, cnum: CrateNum) -> DefIdMap<String> {
    // Build up a map from DefId to a `NativeLib` structure, where
    // `NativeLib` internally contains information about
    // `#[link(wasm_import_module = "...")]` for example.
    let native_libs = tcx.native_libraries(cnum);

    let def_id_to_native_lib = native_libs
        .iter()
        .filter_map(|lib| lib.foreign_module.map(|id| (id, lib)))
        .collect::<DefIdMap<_>>();

    let mut ret = DefIdMap::default();
    for (def_id, lib) in tcx.foreign_modules(cnum).iter() {
        let module = def_id_to_native_lib.get(def_id).and_then(|s| s.wasm_import_module());
        let Some(module) = module else { continue };
        ret.extend(lib.foreign_items.iter().map(|id| {
            assert_eq!(id.krate, cnum);
            (*id, module.to_string())
        }));
    }

    ret
}
```

---
*Generated by AST tracing system*
