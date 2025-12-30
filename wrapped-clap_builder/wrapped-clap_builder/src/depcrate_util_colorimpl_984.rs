// Generated macro for impl_984 (impl)
macro_rules! Depcrate_util_colorimpl_984 {
() => {
// Module: crate::util::color
// Provides: {"impl_984"}
// Dependencies: {}
impl std :: str :: FromStr for ColorChoice { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { for variant in Self :: value_variants () { if variant . to_possible_value () . unwrap () . matches (s , false) { return Ok (* variant) ; } } Err (format ! ("invalid variant: {s}")) } }
};
}
