// Generated macro for impl_96 (impl)
macro_rules! Depcrate_util_secondsimpl_96 {
() => {
// Module: crate::util::seconds
// Provides: {"impl_96"}
// Dependencies: {}
impl super :: TryFromValues for Seconds { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . just_one () . and_then (Seconds :: from_val) . ok_or_else (Error :: invalid) } }
};
}
