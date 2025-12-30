// Generated macro for powerset (function)
macro_rules! Depcrate_powersetpowerset {
() => {
// Module: crate::powerset
// Provides: {"powerset"}
// Dependencies: {}
# [doc = " Create a new `Powerset` from a cloneable iterator."] pub fn powerset < I > (src : I) -> Powerset < I > where I : Iterator , I :: Item : Clone , { Powerset { combs : combinations (src , 0) , } }
};
}
