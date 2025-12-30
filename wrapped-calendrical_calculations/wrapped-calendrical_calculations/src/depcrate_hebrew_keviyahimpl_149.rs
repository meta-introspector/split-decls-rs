// Generated macro for impl_149 (impl)
macro_rules! Depcrate_hebrew_keviyahimpl_149 {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"impl_149"}
// Dependencies: {}
impl MetonicCycleType { fn for_h_year (h_year : i32) -> Self { let remainder = h_year . rem_euclid (19) ; match remainder { 2 | 5 | 10 | 13 | 16 => Self :: LMinusOne , 1 | 4 | 9 | 12 | 15 => Self :: LPlusOne , 7 | 18 => Self :: LPlusMinusOne , _ => { debug_assert ! (matches ! (remainder , 3 | 6 | 8 | 11 | 14 | 17 | 0 | 19)) ; Self :: Leap } } } }
};
}
