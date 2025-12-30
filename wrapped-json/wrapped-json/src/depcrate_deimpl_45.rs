// Generated macro for impl_45 (impl)
macro_rules! Depcrate_deimpl_45 {
() => {
// Module: crate::de
// Provides: {"impl_45"}
// Dependencies: {}
impl FromStr for Number { type Err = Error ; fn from_str (s : & str) -> result :: Result < Self , Self :: Err > { Deserializer :: from_str (s) . parse_any_signed_number () . map (Into :: into) } }
};
}
