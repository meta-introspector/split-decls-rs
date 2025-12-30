// Generated macro for unlikely (function)
macro_rules! Depcrate_utilunlikely {
() => {
// Module: crate::util
// Provides: {"unlikely"}
// Dependencies: {}
# [cfg (not (feature = "nightly"))] # [inline (always)] pub (crate) fn unlikely (b : bool) -> bool { if b { cold_path () ; true } else { false } }
};
}
