// Generated macro for allocator_kind_for_codegen (function)
macro_rules! Depcrate_baseallocator_kind_for_codegen {
() => {
// Module: crate::base
// Provides: {"allocator_kind_for_codegen"}
// Dependencies: {}
# [doc = " Decide allocator kind to codegen. If `Some(_)` this will be the same as"] # [doc = " `tcx.allocator_kind`, but it may be `None` in more cases (e.g. if using"] # [doc = " allocator definitions from a dylib dependency)."] pub fn allocator_kind_for_codegen (tcx : TyCtxt < '_ >) -> Option < AllocatorKind > { let all_crate_types_any_dynamic_crate = tcx . dependency_formats (()) . iter () . all (| (_ , list) | { use rustc_middle :: middle :: dependency_format :: Linkage ; list . iter () . any (| & linkage | linkage == Linkage :: Dynamic) }) ; if all_crate_types_any_dynamic_crate { None } else { tcx . allocator_kind (()) } }
};
}
