// Generated macro for clamp_min (function)
macro_rules! Depcrateclamp_min {
() => {
// Module: crate
// Provides: {"clamp_min"}
// Dependencies: {}
# [doc = " A value bounded by a minimum value"] # [doc = ""] # [doc = "  If input is less than min then this returns min."] # [doc = "  Otherwise this returns input."] # [doc = "  `clamp_min(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::min(std::f32::NAN, 1.0)`."] # [doc = ""] # [doc = " **Panics** in debug mode if `!(min == min)`. (This occurs if `min` is `NAN`.)"] # [inline] # [allow (clippy :: eq_op)] pub fn clamp_min < T : PartialOrd > (input : T , min : T) -> T { debug_assert ! (min == min , "min must not be NAN") ; if input < min { min } else { input } }
};
}
