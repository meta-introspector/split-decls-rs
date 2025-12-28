macro_rules! deps {
    () => {
        VecType!();
        Limb!();
    };
}

macro_rules! shl_bits {
    () => {
        deps!();
        # [doc = " Shift-left `n` bits inside a buffer."] # [inline] pub fn shl_bits (x : & mut VecType , n : usize) -> Option < () > { debug_assert ! (n != 0) ; debug_assert ! (n < LIMB_BITS) ; let rshift = LIMB_BITS - n ; let lshift = n ; let mut prev : Limb = 0 ; for xi in x . iter_mut () { let tmp = * xi ; * xi <<= lshift ; * xi |= prev >> rshift ; prev = tmp ; } let carry = prev >> rshift ; if carry != 0 { x . try_push (carry) ? ; } Some (()) }
    };
}

shl_bits!()