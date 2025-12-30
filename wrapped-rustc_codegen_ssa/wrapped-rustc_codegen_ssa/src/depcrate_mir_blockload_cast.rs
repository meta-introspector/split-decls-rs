// Generated macro for load_cast (function)
macro_rules! Depcrate_mir_blockload_cast {
() => {
// Module: crate::mir::block
// Provides: {"load_cast"}
// Dependencies: {}
fn load_cast < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , cast : & CastTarget , ptr : Bx :: Value , align : Align ,) -> Bx :: Value { let cast_ty = bx . cast_backend_type (cast) ; if let Some (offset_from_start) = cast . rest_offset { assert ! (cast . prefix [1 ..] . iter () . all (| p | p . is_none ())) ; assert_eq ! (cast . rest . unit . size , cast . rest . total) ; let first_ty = bx . reg_backend_type (& cast . prefix [0] . unwrap ()) ; let second_ty = bx . reg_backend_type (& cast . rest . unit) ; let first = bx . load (first_ty , ptr , align) ; let second_ptr = bx . inbounds_ptradd (ptr , bx . const_usize (offset_from_start . bytes ())) ; let second = bx . load (second_ty , second_ptr , align . restrict_for_offset (offset_from_start)) ; let res = bx . cx () . const_poison (cast_ty) ; let res = bx . insert_value (res , first , 0) ; bx . insert_value (res , second , 1) } else { bx . load (cast_ty , ptr , align) } }
};
}
