// Generated macro for impl_27 (impl)
macro_rules! Depcrate_bundle_write_typesimpl_27 {
() => {
// Module: crate::bundle::write::types
// Provides: {"impl_27"}
// Dependencies: {}
impl Outcome { # [doc = " Instantiate a bundle from the newly written index and data file that are represented by this `Outcome`"] pub fn to_bundle (& self) -> Option < Result < crate :: Bundle , crate :: bundle :: init :: Error > > { self . index_path . as_ref () . map (| path | crate :: Bundle :: at (path , self . object_hash)) } }
};
}
