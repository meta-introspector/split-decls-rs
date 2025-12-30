// Generated macro for to_vec (function)
macro_rules! Depcrate_serto_vec {
() => {
// Module: crate::ser
// Provides: {"to_vec"}
// Dependencies: {}
# [doc = " Serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing"] # [doc = " data in a serialized format."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard2::to_vec;"] # [doc = " use heapless::Vec;"] # [doc = " use core::ops::Deref;"] # [doc = ""] # [doc = " let ser: Vec<u8, 32> = to_vec(&true).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x01]);"] # [doc = ""] # [doc = " let ser: Vec<u8, 32> = to_vec(\"Hi!\").unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x03, b'H', b'i', b'!']);"] # [doc = ""] # [doc = " // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently."] # [doc = " let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let ser: Vec<u8, 32> = to_vec(data).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x04, 0x01, 0x00, 0x20, 0x30]);"] # [doc = ""] # [doc = " let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];"] # [doc = " let ser: Vec<u8, 32> = to_vec(data).unwrap();"] # [doc = " assert_eq!(ser.deref(), &[0x01, 0x00, 0x20, 0x30]);"] # [doc = " ```"] # [cfg (feature = "heapless")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless")))] pub fn to_vec < T , const B : usize > (value : & T) -> Result < Vec < u8 , B > > where T : Serialize + ? Sized , { serialize_with_flavor :: < T , HVec < B > , Vec < u8 , B > > (value , HVec :: default ()) }
};
}
