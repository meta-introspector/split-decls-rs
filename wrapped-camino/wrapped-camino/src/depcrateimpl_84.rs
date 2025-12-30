// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl FromStr for Utf8PathBuf { type Err = Infallible ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (Utf8PathBuf (s . into ())) } }
};
}
