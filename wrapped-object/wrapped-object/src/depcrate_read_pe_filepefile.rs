// Generated macro for PeFile (struct)
macro_rules! Depcrate_read_pe_filePeFile {
() => {
// Module: crate::read::pe::file
// Provides: {"PeFile"}
// Dependencies: {}
# [doc = " A PE image file."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] # [derive (Debug)] pub struct PeFile < 'data , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { pub (super) dos_header : & 'data pe :: ImageDosHeader , pub (super) nt_headers : & 'data Pe , pub (super) data_directories : DataDirectories < 'data > , pub (super) common : CoffCommon < 'data , R > , pub (super) data : R , }
};
}
