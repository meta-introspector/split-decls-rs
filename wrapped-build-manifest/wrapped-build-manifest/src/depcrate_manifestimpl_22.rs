// Generated macro for impl_22 (impl)
macro_rules! Depcrate_manifestimpl_22 {
() => {
// Module: crate::manifest
// Provides: {"impl_22"}
// Dependencies: {}
impl Manifest { pub (crate) fn add_artifact (& mut self , name : & str , f : impl FnOnce (& mut Artifact)) { let mut artifact = Artifact { target : BTreeMap :: new () } ; f (& mut artifact) ; self . artifacts . insert (name . to_string () , artifact) ; } }
};
}
