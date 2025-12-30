// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < T : FloatCore > Ord for OrderedFloat < T > { # [inline] fn cmp (& self , other : & Self) -> Ordering { # [allow (clippy :: comparison_chain)] if self < other { Ordering :: Less } else if self > other { Ordering :: Greater } else { Ordering :: Equal } } }
};
}
