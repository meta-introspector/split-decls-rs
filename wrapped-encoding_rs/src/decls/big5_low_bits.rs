macro_rules! big5_low_bits {
    () => {
        # [inline (always)] pub fn big5_low_bits (rebased_pointer : usize) -> u16 { if rebased_pointer < BIG5_LOW_BITS . len () { BIG5_LOW_BITS [rebased_pointer] } else { 0 } }
    };
}

big5_low_bits!()