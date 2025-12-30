// Generated macro for FileHeader (struct)
macro_rules! Depcrate_write_coff_writerFileHeader {
() => {
// Module: crate::write::coff::writer
// Provides: {"FileHeader"}
// Dependencies: {}
# [doc = " Shortened and native endian version of [`pe::ImageFileHeader`]."] # [allow (missing_docs)] # [derive (Debug , Default , Clone)] pub struct FileHeader { pub machine : u16 , pub time_date_stamp : u32 , pub characteristics : u16 , }
};
}
