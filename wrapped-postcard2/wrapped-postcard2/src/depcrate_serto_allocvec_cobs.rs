// Generated macro for to_allocvec_cobs (function)
macro_rules! Depcrate_serto_allocvec_cobs {
() => {
// Module: crate::ser
// Provides: {"to_allocvec_cobs"}
// Dependencies: {}
# [doc = " Serialize and COBS encode a `T` to an `alloc::vec::Vec<u8>`."] # [doc = ""] # [doc = " The terminating sentinel `0x00` byte is included in the output."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard2::to_allocvec_cobs;"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_allocvec_cobs(&true).unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x02, 0x01, 0x00]);"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_allocvec_cobs(\"Hi!\").unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn to_allocvec_cobs < T > (value : & T) -> Result < alloc :: vec :: Vec < u8 > > where T : Serialize + ? Sized , { serialize_with_flavor :: < T , Cobs < AllocVec > , alloc :: vec :: Vec < u8 > > (value , Cobs :: try_new (AllocVec :: new ()) ? ,) }
};
}
