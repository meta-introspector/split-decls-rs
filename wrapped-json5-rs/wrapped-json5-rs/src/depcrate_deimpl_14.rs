// Generated macro for impl_14 (impl)
macro_rules! Depcrate_deimpl_14 {
() => {
// Module: crate::de
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'de > Deserializer < 'de > { # [doc = " Creates a JSON5 deserializer from a `&str`. This parses the input at construction time, so"] # [doc = " can fail if the input is not valid JSON5."] pub fn from_str (input : & 'de str) -> Result < Self > { let pair = Parser :: parse (Rule :: text , input) ? . next () . unwrap () ; Ok (Deserializer :: from_pair (pair)) } fn from_pair (pair : Pair < 'de , Rule >) -> Self { Deserializer { pair : Some (pair) } } }
};
}
