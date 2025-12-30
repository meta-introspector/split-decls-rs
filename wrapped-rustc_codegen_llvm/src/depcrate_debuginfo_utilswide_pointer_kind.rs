// Generated macro for wide_pointer_kind (function)
macro_rules! Depcrate_debuginfo_utilswide_pointer_kind {
() => {
// Module: crate::debuginfo::utils
// Provides: {"wide_pointer_kind"}
// Dependencies: {}
# [doc = " Determines if `pointee_ty` is slice-like or trait-object-like, i.e."] # [doc = " if the second field of the wide pointer is a length or a vtable-pointer."] # [doc = " If `pointee_ty` does not require a wide pointer (because it is Sized) then"] # [doc = " the function returns `None`."] pub (crate) fn wide_pointer_kind < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , pointee_ty : Ty < 'tcx > ,) -> Option < WidePtrKind > { let pointee_tail_ty = cx . tcx . struct_tail_for_codegen (pointee_ty , cx . typing_env ()) ; let layout = cx . layout_of (pointee_tail_ty) ; trace ! ("wide_pointer_kind: {:?} has layout {:?} (is_unsized? {})" , pointee_tail_ty , layout , layout . is_unsized ()) ; if layout . is_sized () { return None ; } match * pointee_tail_ty . kind () { ty :: Str | ty :: Slice (_) => Some (WidePtrKind :: Slice) , ty :: Dynamic (..) => Some (WidePtrKind :: Dyn) , ty :: Foreign (_) => { assert_eq ! (cx . size_of (Ty :: new_imm_ptr (cx . tcx , pointee_tail_ty)) , cx . size_of (Ty :: new_imm_ptr (cx . tcx , cx . tcx . types . u8))) ; None } _ => { panic ! ("wide_pointer_kind() - Encountered unexpected `pointee_tail_ty`: {pointee_tail_ty:?}") } } }
};
}
