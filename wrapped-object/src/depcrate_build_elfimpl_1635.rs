// Generated macro for impl_1635 (impl)
macro_rules! Depcrate_build_elfimpl_1635 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1635"}
// Dependencies: {}
impl < 'data > VersionFiles < 'data > { # [doc = " Add a new filename to the table."] pub fn add (& mut self , name : ByteString < 'data >) -> VersionFileId { let id = self . next_id () ; self . push (VersionFile { id , name , delete : false , }) ; id } }
};
}
