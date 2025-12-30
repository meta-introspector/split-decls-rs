// Generated macro for is_small (function)
macro_rules! Depcrate_anyis_small {
() => {
// Module: crate::any
// Provides: {"is_small"}
// Dependencies: {}
fn is_small < T > () -> bool { mem :: size_of :: < T > () <= mem :: size_of :: < Value > () && mem :: align_of :: < T > () <= mem :: align_of :: < Value > () }
};
}
