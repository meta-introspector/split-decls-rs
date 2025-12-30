// Generated macro for recursively_inline (function)
macro_rules! Depcrate_attributesrecursively_inline {
() => {
// Module: crate::attributes
// Provides: {"recursively_inline"}
// Dependencies: {}
# [doc = " Checks if the function `instance` is recursively inline."] # [doc = " Returns `false` if a functions is guaranteed to be non-recursive, and `true` if it *might* be recursive."] # [cfg (feature = "master")] fn recursively_inline < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , instance : ty :: Instance < 'tcx > ,) -> bool { if ! cx . tcx . is_mir_available (instance . def_id ()) { return true ; } let body = cx . tcx . optimized_mir (instance . def_id ()) ; for block in body . basic_blocks . iter () { let Some (ref terminator) = block . terminator else { continue } ; let TerminatorKind :: Call { ref func , .. } = terminator . kind else { continue } ; let Some ((def , _args)) = func . const_fn_def () else { continue } ; if matches ! (cx . tcx . codegen_fn_attrs (def) . inline , InlineAttr :: Always | InlineAttr :: Force { .. }) { return true ; } } false }
};
}
