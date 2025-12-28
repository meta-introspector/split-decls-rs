macro_rules! rshift {
    () => {
        # [doc = " Constant-time bitwise right shift."] # [inline] pub (crate) fn rshift (n : & mut [u8] , shift : u32) { debug_assert ! (shift < 8) ; let mask = (1 << shift) - 1 ; let mut carry = 0 ; for byte in n . iter_mut () { let new_carry = (* byte & mask) << (8 - shift) ; * byte = (* byte >> shift) | carry ; carry = new_carry ; } }
    };
}

rshift!()