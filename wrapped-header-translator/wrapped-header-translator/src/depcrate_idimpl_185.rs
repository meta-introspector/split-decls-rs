// Generated macro for impl_185 (impl)
macro_rules! Depcrate_idimpl_185 {
() => {
// Module: crate::id
// Provides: {"impl_185"}
// Dependencies: {}
impl FromStr for Location { type Err = Box < dyn Error > ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if s . contains ("::") { return Err (Box :: new (std :: io :: Error :: other ("requires ., not ::"))) ; } Ok (Location { module_path : s . into () , }) } }
};
}
