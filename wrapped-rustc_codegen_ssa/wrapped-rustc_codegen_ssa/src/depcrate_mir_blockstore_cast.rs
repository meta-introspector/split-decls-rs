// Generated macro for store_cast (function)
macro_rules! Depcrate_mir_blockstore_cast {
() => {
// Module: crate::mir::block
// Provides: {"store_cast"}
// Dependencies: {}
pub fn store_cast < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , cast : & CastTarget , value : Bx :: Value , ptr : Bx :: Value , align : Align ,) { if let Some (offset_from_start) = cast . rest_offset { assert ! (cast . prefix [1 ..] . iter () . all (| p | p . is_none ())) ; assert_eq ! (cast . rest . unit . size , cast . rest . total) ; assert ! (cast . prefix [0] . is_some ()) ; let first = bx . extract_value (value , 0) ; let second = bx . extract_value (value , 1) ; bx . store (first , ptr , align) ; let second_ptr = bx . inbounds_ptradd (ptr , bx . const_usize (offset_from_start . bytes ())) ; bx . store (second , second_ptr , align . restrict_for_offset (offset_from_start)) ; } else { bx . store (value , ptr , align) ; } ; }
};
}
