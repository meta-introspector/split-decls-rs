// Generated macro for get_slice_like_element_ty (function)
macro_rules! Depcrate_loops_manual_memcpyget_slice_like_element_ty {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"get_slice_like_element_ty"}
// Dependencies: {}
fn get_slice_like_element_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < Ty < 'tcx > > { match ty . kind () { ty :: Adt (adt , subs) if cx . tcx . is_diagnostic_item (sym :: Vec , adt . did ()) => Some (subs . type_at (0)) , ty :: Ref (_ , subty , _) => get_slice_like_element_ty (cx , * subty) , ty :: Slice (ty) | ty :: Array (ty , _) => Some (* ty) , _ => None , } }
};
}
