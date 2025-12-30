// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl FromStr for OutputFormat { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let s_lc = s . to_ascii_lowercase () ; Ok (match s_lc . as_str () { "human" => OutputFormat :: Human , # [cfg (feature = "serde")] "json" => OutputFormat :: Json , _ => return Err (format ! ("Invalid output format: '{s}'")) , }) } }
};
}
