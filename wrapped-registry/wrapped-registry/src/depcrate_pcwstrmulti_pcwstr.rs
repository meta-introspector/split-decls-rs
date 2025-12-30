// Generated macro for multi_pcwstr (function)
macro_rules! Depcrate_pcwstrmulti_pcwstr {
() => {
// Module: crate::pcwstr
// Provides: {"multi_pcwstr"}
// Dependencies: {}
pub fn multi_pcwstr < T : AsRef < str > > (value : & [T]) -> OwnedPcwstr { OwnedPcwstr (value . iter () . flat_map (| value | value . as_ref () . encode_utf16 () . chain (core :: iter :: once (0))) . chain (core :: iter :: once (0)) . collect () ,) }
};
}
