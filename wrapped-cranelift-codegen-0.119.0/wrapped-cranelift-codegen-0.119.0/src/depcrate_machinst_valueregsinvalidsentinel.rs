// Generated macro for InvalidSentinel (trait)
macro_rules! Depcrate_machinst_valueregsInvalidSentinel {
() => {
// Module: crate::machinst::valueregs
// Provides: {"InvalidSentinel"}
// Dependencies: {}
# [doc = " A type with an \"invalid\" sentinel value."] pub trait InvalidSentinel : Copy + Eq { # [doc = " The invalid sentinel value."] fn invalid_sentinel () -> Self ; # [doc = " Is this the invalid sentinel?"] fn is_invalid_sentinel (self) -> bool { self == Self :: invalid_sentinel () } }
};
}
