// Generated macro for to_slice_crc32 (function)
macro_rules! Depcrate_serto_slice_crc32 {
() => {
// Module: crate::ser
// Provides: {"to_slice_crc32"}
// Dependencies: {}
# [doc = " Conveniently serialize a `T` to the given slice, with the resulting slice containing"] # [doc = " data followed by a 32-bit CRC. The CRC bytes are included in the output buffer."] # [doc = ""] # [doc = " When successful, this function returns the slice containing the"] # [doc = " serialized and encoded message."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use crc::{Crc, CRC_32_ISCSI};"] # [doc = ""] # [doc = " let mut buf = [0; 9];"] # [doc = ""] # [doc = " let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];"] # [doc = " let crc = Crc::<u32>::new(&CRC_32_ISCSI);"] # [doc = " let used = postcard2::to_slice_crc32(data, &mut buf, crc.digest()).unwrap();"] # [doc = " assert_eq!(used, &[0x04, 0x01, 0x00, 0x20, 0x30, 0x8E, 0xC8, 0x1A, 0x37]);"] # [doc = " ```"] # [doc = ""] # [doc = " See the `ser_flavors::crc` module for the complete set of functions."] # [cfg (feature = "use-crc")] # [cfg_attr (docsrs , doc (cfg (feature = "use-crc")))] # [inline] pub fn to_slice_crc32 < 'a , T > (value : & T , buf : & 'a mut [u8] , digest : crc :: Digest < '_ , u32 > ,) -> Result < & 'a mut [u8] > where T : Serialize + ? Sized , { flavors :: crc :: to_slice_u32 (value , buf , digest) }
};
}
