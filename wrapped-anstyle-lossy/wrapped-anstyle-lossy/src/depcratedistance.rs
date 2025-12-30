// Generated macro for distance (function)
macro_rules! Depcratedistance {
() => {
// Module: crate
// Provides: {"distance"}
// Dependencies: {}
# [doc = " Low-cost approximation from <https://www.compuphase.com/cmetric.htm>, modified to avoid sqrt"] pub (crate) const fn distance (c1 : anstyle :: RgbColor , c2 : anstyle :: RgbColor) -> u32 { let c1_r = c1 . r () as i32 ; let c1_g = c1 . g () as i32 ; let c1_b = c1 . b () as i32 ; let c2_r = c2 . r () as i32 ; let c2_g = c2 . g () as i32 ; let c2_b = c2 . b () as i32 ; let r_sum = c1_r + c2_r ; let r_delta = c1_r - c2_r ; let g_delta = c1_g - c2_g ; let b_delta = c1_b - c2_b ; let r = (2 * 512 + r_sum) * r_delta * r_delta ; let g = 4 * g_delta * g_delta * (1 << 8) ; let b = (2 * 767 - r_sum) * b_delta * b_delta ; (r + g + b) as u32 }
};
}
