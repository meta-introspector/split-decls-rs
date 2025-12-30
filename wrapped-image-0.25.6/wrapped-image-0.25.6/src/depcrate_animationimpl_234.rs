// Generated macro for impl_234 (impl)
macro_rules! Depcrate_animationimpl_234 {
() => {
// Module: crate::animation
// Provides: {"impl_234"}
// Dependencies: {}
impl Ord for Ratio { fn cmp (& self , other : & Self) -> Ordering { let a : u32 = self . numer ; let b : u32 = self . denom ; let c : u32 = other . numer ; let d : u32 = other . denom ; (u64 :: from (a) * u64 :: from (d)) . cmp (& (u64 :: from (c) * u64 :: from (b))) } }
};
}
