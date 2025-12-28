macro_rules! FloatAsSIMD {
    () => {
        # [doc = " Implement functions on f32/f64 to give them APIs similar to SIMD types"] pub (crate) trait FloatAsSIMD : Sized { # [cfg (test)] const LEN : usize = 1 ; # [inline (always)] fn splat (scalar : Self) -> Self { scalar } }
    };
}

FloatAsSIMD!();