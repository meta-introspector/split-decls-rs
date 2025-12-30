// Generated macro for clamp_max (function)
macro_rules! Depcrateclamp_max {
() => {
// Module: crate
// Provides: {"clamp_max"}
// Dependencies: {}
# [doc = " A value bounded by a maximum value"] # [doc = ""] # [doc = "  If input is greater than max then this returns max."] # [doc = "  Otherwise this returns input."] # [doc = "  `clamp_max(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::max(std::f32::NAN, 1.0)`."] # [doc = ""] # [doc = " **Panics** in debug mode if `!(max == max)`. (This occurs if `max` is `NAN`.)"] # [inline] # [allow (clippy :: eq_op)] pub fn clamp_max < T : PartialOrd > (input : T , max : T) -> T { debug_assert ! (max == max , "max must not be NAN") ; if input > max { max } else { input } }
};
}
