// Generated macro for impl_1650 (impl)
macro_rules! Depcrate_build_elfimpl_1650 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1650"}
// Dependencies: {}
impl < 'data > Versions < 'data > { # [doc = " Add a version."] pub fn add (& mut self , data : VersionData < 'data >) -> VersionId { let id = self . next_id () ; self . push (Version { id , data , delete : false , }) ; id } }
};
}
