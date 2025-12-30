// Generated macro for coerce_unsized_into (function)
macro_rules! Depcrate_unsizecoerce_unsized_into {
() => {
// Module: crate::unsize
// Provides: {"coerce_unsized_into"}
// Dependencies: {}
# [doc = " Coerce `src`, which is a reference to a value of type `src_ty`,"] # [doc = " to a value of type `dst_ty` and store the result in `dst`"] pub (crate) fn coerce_unsized_into < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , src : CValue < 'tcx > , dst : CPlace < 'tcx > ,) { let src_ty = src . layout () . ty ; let dst_ty = dst . layout () . ty ; let mut coerce_ptr = | | { let (base , info) = if fx . layout_of (src . layout () . ty . builtin_deref (true) . unwrap ()) . is_unsized () { let (old_base , old_info) = src . load_scalar_pair (fx) ; unsize_ptr (fx , old_base , src . layout () , dst . layout () , Some (old_info)) } else { let base = src . load_scalar (fx) ; unsize_ptr (fx , base , src . layout () , dst . layout () , None) } ; dst . write_cvalue (fx , CValue :: by_val_pair (base , info , dst . layout ())) ; } ; match (& src_ty . kind () , & dst_ty . kind ()) { (& ty :: Ref (..) , & ty :: Ref (..)) | (& ty :: Ref (..) , & ty :: RawPtr (..)) | (& ty :: RawPtr (..) , & ty :: RawPtr (..)) => coerce_ptr () , (& ty :: Adt (def_a , _) , & ty :: Adt (def_b , _)) => { assert_eq ! (def_a , def_b) ; for i in 0 .. def_a . variant (FIRST_VARIANT) . fields . len () { let src_f = src . value_field (fx , FieldIdx :: new (i)) ; let dst_f = dst . place_field (fx , FieldIdx :: new (i)) ; if dst_f . layout () . is_zst () { continue ; } if src_f . layout () . ty == dst_f . layout () . ty { dst_f . write_cvalue (fx , src_f) ; } else { coerce_unsized_into (fx , src_f , dst_f) ; } } } _ => bug ! ("coerce_unsized_into: invalid coercion {:?} -> {:?}" , src_ty , dst_ty) , } }
};
}
