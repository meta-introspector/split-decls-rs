macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! bit_length {
    () => {
        deps!();
        # [doc = " Calculate the bit-length of the big-integer."] # [inline] pub fn bit_length (x : & [Limb]) -> u32 { let nlz = leading_zeros (x) ; LIMB_BITS as u32 * x . len () as u32 - nlz }
    };
}

bit_length!()