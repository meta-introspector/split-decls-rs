// Generated macro for next (function)
macro_rules! Depcrate_helpersnext {
() => {
// Module: crate::helpers
// Provides: {"next"}
// Dependencies: {}
pub (crate) fn next < F > (mut index : RataDie , condition : F) -> RataDie where F : Fn (RataDie) -> bool , { loop { if condition (index) { return index ; } index += 1 ; } }
};
}
