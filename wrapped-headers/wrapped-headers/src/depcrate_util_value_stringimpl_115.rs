// Generated macro for impl_115 (impl)
macro_rules! Depcrate_util_value_stringimpl_115 {
() => {
// Module: crate::util::value_string
// Provides: {"impl_115"}
// Dependencies: {}
impl FromStr for HeaderValueString { type Err = FromStrError ; fn from_str (src : & str) -> Result < Self , Self :: Err > { src . parse () . map (| value | HeaderValueString { value }) . map_err (| _ | FromStrError (())) } }
};
}
