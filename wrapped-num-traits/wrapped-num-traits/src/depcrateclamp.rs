// Generated macro for clamp (function)
macro_rules! Depcrateclamp {
() => {
// Module: crate
// Provides: {"clamp"}
// Dependencies: {}
# [doc = " A value bounded by a minimum and a maximum"] # [doc = ""] # [doc = "  If input is less than min then this returns min."] # [doc = "  If input is greater than max then this returns max."] # [doc = "  Otherwise this returns input."] # [doc = ""] # [doc = " **Panics** in debug mode if `!(min <= max)`."] # [inline] pub fn clamp < T : PartialOrd > (input : T , min : T , max : T) -> T { debug_assert ! (min <= max , "min must be less than or equal to max") ; if input < min { min } else if input > max { max } else { input } }
};
}
