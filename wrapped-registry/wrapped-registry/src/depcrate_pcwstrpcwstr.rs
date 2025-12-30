// Generated macro for pcwstr (function)
macro_rules! Depcrate_pcwstrpcwstr {
() => {
// Module: crate::pcwstr
// Provides: {"pcwstr"}
// Dependencies: {}
pub fn pcwstr < T : AsRef < str > > (value : T) -> OwnedPcwstr { OwnedPcwstr (value . as_ref () . encode_utf16 () . chain (core :: iter :: once (0)) . collect () ,) }
};
}
