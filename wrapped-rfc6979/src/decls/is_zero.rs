macro_rules! is_zero {
    () => {
        # [doc = " Constant-time test that a given byte slice contains only zeroes."] # [inline] pub (crate) fn is_zero (n : & [u8]) -> Choice { let mut ret = Choice :: from (1) ; for byte in n { ret . conditional_assign (& Choice :: from (0) , byte . ct_ne (& 0)) ; } ret }
    };
}

is_zero!()