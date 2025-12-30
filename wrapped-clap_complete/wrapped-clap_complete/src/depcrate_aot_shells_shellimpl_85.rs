// Generated macro for impl_85 (impl)
macro_rules! Depcrate_aot_shells_shellimpl_85 {
() => {
// Module: crate::aot::shells::shell
// Provides: {"impl_85"}
// Dependencies: {}
impl FromStr for Shell { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { for variant in Self :: value_variants () { if variant . to_possible_value () . unwrap () . matches (s , false) { return Ok (* variant) ; } } Err (format ! ("invalid variant: {s}")) } }
};
}
