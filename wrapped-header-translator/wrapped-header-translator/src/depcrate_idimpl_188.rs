// Generated macro for impl_188 (impl)
macro_rules! Depcrate_idimpl_188 {
() => {
// Module: crate::id
// Provides: {"impl_188"}
// Dependencies: {}
impl FromStr for ItemIdentifier { type Err = Box < dyn Error > ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if s . contains ("::") { return Err (Box :: new (std :: io :: Error :: other ("requires ., not ::"))) ; } let (module_path , name) = s . rsplit_once ('.') . ok_or_else (| | std :: io :: Error :: other ("requires at least one .")) ? ; Ok (Self { name : name . into () , location : Location { module_path : module_path . into () , } , }) } }
};
}
