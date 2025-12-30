// Generated macro for clamp (function)
macro_rules! Depcrateclamp {
() => {
// Module: crate
// Provides: {"clamp"}
// Dependencies: {}
# [doc = " Returns a reference to the input value clamped to the interval `[min, max]`."] # [doc = ""] # [doc = " In particular:"] # [doc = "     * If `min < val < max`, this returns `val`."] # [doc = "     * If `val <= min`, this returns `min`."] # [doc = "     * If `val >= max`, this returns `max`."] # [must_use] # [inline] pub fn clamp < T : PartialOrd > (val : T , min : T , max : T) -> T { if val > min { if val < max { val } else { max } } else { min } }
};
}
