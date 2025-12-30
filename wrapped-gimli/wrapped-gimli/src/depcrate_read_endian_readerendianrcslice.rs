// Generated macro for EndianRcSlice (type)
macro_rules! Depcrate_read_endian_readerEndianRcSlice {
() => {
// Module: crate::read::endian_reader
// Provides: {"EndianRcSlice"}
// Dependencies: {}
# [doc = " A reference counted, non-thread-safe slice of bytes and associated"] # [doc = " endianity."] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"std\")] {"] # [doc = " use std::rc::Rc;"] # [doc = ""] # [doc = " let buf = Rc::from(&[1, 2, 3, 4][..]);"] # [doc = " let reader = gimli::EndianRcSlice::new(buf, gimli::NativeEndian);"] # [doc = " # let _ = reader;"] # [doc = " # }"] # [doc = " ```"] pub type EndianRcSlice < Endian > = EndianReader < Endian , Rc < [u8] > > ;
};
}
