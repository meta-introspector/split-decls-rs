// Generated macro for to_allocvec (function)
macro_rules! Depcrate_serto_allocvec {
() => {
// Module: crate::ser
// Provides: {"to_allocvec"}
// Dependencies: {}
# [doc = " Serialize a `T` to an `alloc::vec::Vec<u8>`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard::to_allocvec;"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_allocvec(&true).unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x01]);"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_allocvec(\"Hi!\").unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x03, b'H', b'i', b'!']);"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn to_allocvec < T > (value : & T) -> Result < alloc :: vec :: Vec < u8 > > where T : Serialize + ? Sized , { serialize_with_flavor :: < T , AllocVec , alloc :: vec :: Vec < u8 > > (value , AllocVec :: new ()) }
};
}
