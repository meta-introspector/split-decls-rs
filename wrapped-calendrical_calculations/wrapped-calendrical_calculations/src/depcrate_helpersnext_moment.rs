// Generated macro for next_moment (function)
macro_rules! Depcrate_helpersnext_moment {
() => {
// Module: crate::helpers
// Provides: {"next_moment"}
// Dependencies: {}
pub (crate) fn next_moment < F > (mut index : Moment , location : Location , condition : F) -> RataDie where F : Fn (Moment , Location) -> bool , { loop { if condition (index , location) { return index . as_rata_die () ; } index += 1.0 ; } }
};
}
