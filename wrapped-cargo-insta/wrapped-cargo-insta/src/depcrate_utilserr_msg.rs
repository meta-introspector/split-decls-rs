// Generated macro for err_msg (function)
macro_rules! Depcrate_utilserr_msg {
() => {
// Module: crate::utils
// Provides: {"err_msg"}
// Dependencies: {}
pub (crate) fn err_msg < S : Into < String > > (s : S) -> Box < dyn Error > { Box :: new (ErrMsg (s . into ())) }
};
}
