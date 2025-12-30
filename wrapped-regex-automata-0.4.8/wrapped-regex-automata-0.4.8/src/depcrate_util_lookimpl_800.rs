// Generated macro for impl_800 (impl)
macro_rules! Depcrate_util_lookimpl_800 {
() => {
// Module: crate::util::look
// Provides: {"impl_800"}
// Dependencies: {}
impl UnicodeWordBoundaryError { # [cfg (not (feature = "unicode-word-boundary"))] pub (crate) fn new () -> UnicodeWordBoundaryError { UnicodeWordBoundaryError (()) } # [doc = " Returns an error if and only if Unicode word boundary data is"] # [doc = " unavailable."] pub fn check () -> Result < () , UnicodeWordBoundaryError > { is_word_char :: check () } }
};
}
