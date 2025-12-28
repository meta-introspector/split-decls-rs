macro_rules! lt {
    () => {
        # [doc = " Constant-time less than."] # [doc = ""] # [doc = " Inputs are interpreted as big endian integers."] # [inline] pub (crate) fn lt (a : & [u8] , b : & [u8]) -> Choice { debug_assert_eq ! (a . len () , b . len ()) ; let mut borrow = 0 ; for (& a , & b) in a . iter () . zip (b . iter ()) . rev () { let c = (b as u16) . wrapping_add (borrow >> (u8 :: BITS - 1)) ; borrow = (a as u16) . wrapping_sub (c) >> u8 :: BITS as u8 ; } ! borrow . ct_eq (& 0) }
    };
}

lt!()