// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl From < f64 > for f32 { type Output = Result < f32 , Error > ; # [inline] fn cast (src : f64) -> Self :: Output { use core :: { f32 , f64 } ; if src != src || src == f64 :: INFINITY || src == f64 :: NEG_INFINITY { Ok (src as f32) } else if src < f32 :: MIN as f64 { Err (Error :: Underflow) } else if src > f32 :: MAX as f64 { Err (Error :: Overflow) } else { Ok (src as f32) } } }
};
}
