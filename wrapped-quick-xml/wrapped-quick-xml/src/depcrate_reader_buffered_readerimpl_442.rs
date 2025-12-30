// Generated macro for impl_442 (impl)
macro_rules! Depcrate_reader_buffered_readerimpl_442 {
() => {
// Module: crate::reader::buffered_reader
// Provides: {"impl_442"}
// Dependencies: {}
# [doc = " Implementation of `XmlSource` for any `BufRead` reader using a user-given"] # [doc = " `Vec<u8>` as buffer that will be borrowed by events."] impl < 'b , R : BufRead > XmlSource < 'b , & 'b mut Vec < u8 > > for R { impl_buffered_source ! () ; }
};
}
