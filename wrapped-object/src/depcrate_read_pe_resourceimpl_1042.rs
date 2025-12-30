// Generated macro for impl_1042 (impl)
macro_rules! Depcrate_read_pe_resourceimpl_1042 {
() => {
// Module: crate::read::pe::resource
// Provides: {"impl_1042"}
// Dependencies: {}
impl < 'data > ResourceDirectory < 'data > { # [doc = " Construct from the data of the `.rsrc` section."] pub fn new (data : & 'data [u8]) -> Self { ResourceDirectory { data } } # [doc = " Parses the root resource directory."] pub fn root (& self) -> Result < ResourceDirectoryTable < 'data > > { ResourceDirectoryTable :: parse (self . data , 0) } }
};
}
