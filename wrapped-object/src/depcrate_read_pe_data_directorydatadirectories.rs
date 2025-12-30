// Generated macro for DataDirectories (struct)
macro_rules! Depcrate_read_pe_data_directoryDataDirectories {
() => {
// Module: crate::read::pe::data_directory
// Provides: {"DataDirectories"}
// Dependencies: {}
# [doc = " The table of data directories in a PE file."] # [doc = ""] # [doc = " Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse)."] # [derive (Debug , Clone , Copy)] pub struct DataDirectories < 'data > { entries : & 'data [pe :: ImageDataDirectory] , }
};
}
