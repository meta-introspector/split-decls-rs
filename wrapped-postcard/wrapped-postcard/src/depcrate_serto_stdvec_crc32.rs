// Generated macro for to_stdvec_crc32 (function)
macro_rules! Depcrate_serto_stdvec_crc32 {
() => {
// Module: crate::ser
// Provides: {"to_stdvec_crc32"}
// Dependencies: {}
# [doc = " Conveniently serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing"] # [doc = " data followed by a 32-bit  CRC. The CRC bytes are included in the output `Vec`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use crc::{Crc, CRC_32_ISCSI};"] # [doc = " use core::ops::Deref;"] # [doc = ""] # [doc = " // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently."] # [doc = " let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let crc = Crc::<u32>::new(&CRC_32_ISCSI);"] # [doc = " let ser: Vec<u8> = postcard::to_stdvec_crc32(data, crc.digest()).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x04, 0x01, 0x00, 0x20, 0x30, 0x8E, 0xC8, 0x1A, 0x37]);"] # [doc = ""] # [doc = " let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let ser: Vec<u8> = postcard::to_stdvec_crc32(data, crc.digest()).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x01, 0x00, 0x20, 0x30, 0xCC, 0x4B, 0x4A, 0xDA]);"] # [doc = " ```"] # [doc = ""] # [doc = " See the `ser_flavors::crc` module for the complete set of functions."] # [cfg (all (feature = "use-crc" , feature = "use-std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "use-crc" , feature = "use-std"))))] # [inline] pub fn to_stdvec_crc32 < T > (value : & T , digest : crc :: Digest < '_ , u32 >) -> Result < std :: vec :: Vec < u8 > > where T : Serialize + ? Sized , { flavors :: crc :: to_allocvec_u32 (value , digest) }
};
}
