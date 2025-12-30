// Generated macro for impl_119 (impl)
macro_rules! Depcrate_utilimpl_119 {
() => {
// Module: crate::util
// Provides: {"impl_119"}
// Dependencies: {}
impl TryFromValues for HeaderValue { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . next () . cloned () . ok_or_else (Error :: invalid) } }
};
}
