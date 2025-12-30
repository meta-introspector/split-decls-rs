// Generated macro for ResourceDirectory (struct)
macro_rules! Depcrate_read_pe_resourceResourceDirectory {
() => {
// Module: crate::read::pe::resource
// Provides: {"ResourceDirectory"}
// Dependencies: {}
# [doc = " The `.rsrc` section of a PE file."] # [doc = ""] # [doc = " Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory)."] # [derive (Debug , Clone , Copy)] pub struct ResourceDirectory < 'data > { data : & 'data [u8] , }
};
}
