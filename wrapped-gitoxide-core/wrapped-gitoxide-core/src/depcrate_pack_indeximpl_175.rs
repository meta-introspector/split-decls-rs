// Generated macro for impl_175 (impl)
macro_rules! Depcrate_pack_indeximpl_175 {
() => {
// Module: crate::pack::index
// Provides: {"impl_175"}
// Dependencies: {}
impl FromStr for IterationMode { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use IterationMode :: * ; let slc = s . to_ascii_lowercase () ; Ok (match slc . as_str () { "as-is" => AsIs , "verify" => Verify , "restore" => Restore , _ => return Err ("invalid value" . into ()) , }) } }
};
}
