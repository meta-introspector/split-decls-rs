// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl < T : Display + PartialOrd > Pair < T > { fn cmp_display (& self) { if self . x >= self . y { println ! ("The largest member is x = {}" , self . x) ; } else { println ! ("The largest member is y = {}" , self . y) ; } } }
};
}
