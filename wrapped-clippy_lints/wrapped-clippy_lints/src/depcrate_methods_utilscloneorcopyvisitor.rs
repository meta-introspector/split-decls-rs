// Generated macro for CloneOrCopyVisitor (struct)
macro_rules! Depcrate_methods_utilsCloneOrCopyVisitor {
() => {
// Module: crate::methods::utils
// Provides: {"CloneOrCopyVisitor"}
// Dependencies: {}
# [doc = " `clone_or_copy_needed` will be false when `CloneOrCopyVisitor` is done visiting if the only"] # [doc = " operations performed on `binding_hir_ids` are:"] # [doc = " * to take non-mutable references to them"] # [doc = " * to use them as non-mutable `&self` in method calls"] # [doc = ""] # [doc = " If any of `binding_hir_ids` is used in any other way, then `clone_or_copy_needed` will be true"] # [doc = " when `CloneOrCopyVisitor` is done visiting."] struct CloneOrCopyVisitor < 'cx , 'tcx > { cx : & 'cx LateContext < 'tcx > , binding_hir_ids : Vec < HirId > , clone_or_copy_needed : bool , references_to_binding : Vec < (Span , String) > , }
};
}
