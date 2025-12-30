// Generated macro for to_vec_cobs (function)
macro_rules! Depcrate_serto_vec_cobs {
() => {
// Module: crate::ser
// Provides: {"to_vec_cobs"}
// Dependencies: {}
# [doc = " Serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing"] # [doc = " data in a serialized then COBS encoded format. The terminating sentinel"] # [doc = " `0x00` byte is included in the output `Vec`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard2::to_vec_cobs;"] # [doc = " use heapless::Vec;"] # [doc = " use core::ops::Deref;"] # [doc = ""] # [doc = " let ser: Vec<u8, 32> = to_vec_cobs(&false).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x01, 0x01, 0x00]);"] # [doc = ""] # [doc = " let ser: Vec<u8, 32> = to_vec_cobs(\"Hi!\").unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);"] # [doc = ""] # [doc = " // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently."] # [doc = " let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let ser: Vec<u8, 32> = to_vec_cobs(data).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);"] # [doc = ""] # [doc = " let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let ser: Vec<u8, 32> = to_vec_cobs(data).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x02, 0x01, 0x03, 0x20, 0x30, 0x00]);"] # [doc = " ```"] # [cfg (feature = "heapless")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless")))] pub fn to_vec_cobs < T , const B : usize > (value : & T) -> Result < Vec < u8 , B > > where T : Serialize + ? Sized , { serialize_with_flavor :: < T , Cobs < HVec < B > > , Vec < u8 , B > > (value , Cobs :: try_new (HVec :: default ()) ?) }
};
}
