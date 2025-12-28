macro_rules! fabsf {
    () => {
        # [doc = " Absolute value (magnitude) (f32)"] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] pub fn fabsf (x : f32) -> f32 { f32 :: from_bits (x . to_bits () & 0x7fffffff) }
    };
}

fabsf!()