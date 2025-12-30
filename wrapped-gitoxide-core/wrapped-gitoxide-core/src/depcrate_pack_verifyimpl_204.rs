// Generated macro for impl_204 (impl)
macro_rules! Depcrate_pack_verifyimpl_204 {
() => {
// Module: crate::pack::verify
// Provides: {"impl_204"}
// Dependencies: {}
impl FromStr for Algorithm { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let s_lc = s . to_ascii_lowercase () ; Ok (match s_lc . as_str () { "less-memory" => Algorithm :: LessMemory , "less-time" => Algorithm :: LessTime , _ => return Err (format ! ("Invalid verification algorithm: '{s}'")) , }) } }
};
}
