// Generated macro for clamp (function)
macro_rules! Depcrate_utilsclamp {
() => {
// Module: crate::utils
// Provides: {"clamp"}
// Dependencies: {}
# [inline] pub (crate) fn clamp < N > (a : N , min : N , max : N) -> N where N : PartialOrd , { if a < min { min } else if a > max { max } else { a } }
};
}
