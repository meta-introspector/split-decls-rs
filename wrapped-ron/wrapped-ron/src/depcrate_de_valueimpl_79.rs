// Generated macro for impl_79 (impl)
macro_rules! Depcrate_de_valueimpl_79 {
() => {
// Module: crate::de::value
// Provides: {"impl_79"}
// Dependencies: {}
impl core :: str :: FromStr for Value { type Err = crate :: error :: SpannedError ; # [doc = " Creates a value from a string reference."] fn from_str (s : & str) -> SpannedResult < Self > { let mut de = super :: Deserializer :: from_str (s) ? ; let val = Value :: deserialize (& mut de) . map_err (| e | de . span_error (e)) ? ; de . end () . map_err (| e | de . span_error (e)) ? ; Ok (val) } }
};
}
