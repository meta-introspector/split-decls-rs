// Generated macro for to_slice_cobs (function)
macro_rules! Depcrate_serto_slice_cobs {
() => {
// Module: crate::ser
// Provides: {"to_slice_cobs"}
// Dependencies: {}
# [doc = " Serialize a `T` to the given slice, with the resulting slice containing"] # [doc = " data in a serialized then COBS encoded format. The terminating sentinel"] # [doc = " `0x00` byte is included in the output buffer."] # [doc = ""] # [doc = " When successful, this function returns the slice containing the"] # [doc = " serialized and encoded message."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard::to_slice_cobs;"] # [doc = " let mut buf = [0u8; 32];"] # [doc = ""] # [doc = " let used = to_slice_cobs(&false, &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x01, 0x01, 0x00]);"] # [doc = ""] # [doc = " let used = to_slice_cobs(\"1\", &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x03, 0x01, b'1', 0x00]);"] # [doc = ""] # [doc = " let used = to_slice_cobs(\"Hi!\", &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x05, 0x03, b'H', b'i', b'!', 0x00]);"] # [doc = ""] # [doc = " let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let used = to_slice_cobs(data, &mut buf).unwrap();"] # [doc = " assert_eq!(used, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);"] # [doc = " ```"] pub fn to_slice_cobs < 'a , 'b , T > (value : & 'b T , buf : & 'a mut [u8]) -> Result < & 'a mut [u8] > where T : Serialize + ? Sized , { serialize_with_flavor :: < T , Cobs < Slice < 'a > > , & 'a mut [u8] > (value , Cobs :: try_new (Slice :: new (buf)) ? ,) }
};
}
