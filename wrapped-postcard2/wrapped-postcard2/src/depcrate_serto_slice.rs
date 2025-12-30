// Generated macro for to_slice (function)
macro_rules! Depcrate_serto_slice {
() => {
// Module: crate::ser
// Provides: {"to_slice"}
// Dependencies: {}
# [doc = " Serialize a `T` to the given slice, with the resulting slice containing"] # [doc = " data in a serialized format."] # [doc = ""] # [doc = " When successful, this function returns the slice containing the"] # [doc = " serialized message"] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard2::to_slice;"] # [doc = " let mut buf = [0u8; 32];"] # [doc = ""] # [doc = " let used = to_slice(&true, &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x01]);"] # [doc = ""] # [doc = " let used = to_slice(\"Hi!\", &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x03, b'H', b'i', b'!']);"] # [doc = ""] # [doc = " // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently."] # [doc = " let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let used = to_slice(data, &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x04, 0x01, 0x00, 0x20, 0x30]);"] # [doc = ""] # [doc = " let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let used = to_slice(data, &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x01, 0x00, 0x20, 0x30]);"] # [doc = " ```"] pub fn to_slice < 'a , 'b , T > (value : & 'b T , buf : & 'a mut [u8]) -> Result < & 'a mut [u8] > where T : Serialize + ? Sized , { serialize_with_flavor :: < T , Slice < 'a > , & 'a mut [u8] > (value , Slice :: new (buf)) }
};
}
