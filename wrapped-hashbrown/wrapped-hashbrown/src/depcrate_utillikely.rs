// Generated macro for likely (function)
macro_rules! Depcrate_utillikely {
() => {
// Module: crate::util
// Provides: {"likely"}
// Dependencies: {}
# [cfg (not (feature = "nightly"))] # [inline (always)] pub (crate) fn likely (b : bool) -> bool { if b { true } else { cold_path () ; false } }
};
}
