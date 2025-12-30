// Generated macro for coerce_unsized_into (function)
macro_rules! Depcrate_basecoerce_unsized_into {
() => {
// Module: crate::base
// Provides: {"coerce_unsized_into"}
// Dependencies: {}
# [doc = " Coerces `src`, which is a reference to a value of type `src_ty`,"] # [doc = " to a value of type `dst_ty`, and stores the result in `dst`."] pub (crate) fn coerce_unsized_into < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , src : PlaceRef < 'tcx , Bx :: Value > , dst : PlaceRef < 'tcx , Bx :: Value > ,) { let src_ty = src . layout . ty ; let dst_ty = dst . layout . ty ; match (src_ty . kind () , dst_ty . kind ()) { (& ty :: Ref (..) , & ty :: Ref (..) | & ty :: RawPtr (..)) | (& ty :: RawPtr (..) , & ty :: RawPtr (..)) => { let (base , info) = match bx . load_operand (src) . val { OperandValue :: Pair (base , info) => unsize_ptr (bx , base , src_ty , dst_ty , Some (info)) , OperandValue :: Immediate (base) => unsize_ptr (bx , base , src_ty , dst_ty , None) , OperandValue :: Ref (..) | OperandValue :: ZeroSized => bug ! () , } ; OperandValue :: Pair (base , info) . store (bx , dst) ; } (& ty :: Adt (def_a , _) , & ty :: Adt (def_b , _)) => { assert_eq ! (def_a , def_b) ; for i in def_a . variant (FIRST_VARIANT) . fields . indices () { let src_f = src . project_field (bx , i . as_usize ()) ; let dst_f = dst . project_field (bx , i . as_usize ()) ; if dst_f . layout . is_zst () { continue ; } if src_f . layout . ty == dst_f . layout . ty { bx . typed_place_copy (dst_f . val , src_f . val , src_f . layout) ; } else { coerce_unsized_into (bx , src_f , dst_f) ; } } } _ => bug ! ("coerce_unsized_into: invalid coercion {:?} -> {:?}" , src_ty , dst_ty ,) , } }
};
}
