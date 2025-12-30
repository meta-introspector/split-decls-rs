// Generated macro for to_stdvec_cobs (function)
macro_rules! Depcrate_serto_stdvec_cobs {
() => {
// Module: crate::ser
// Provides: {"to_stdvec_cobs"}
// Dependencies: {}
# [doc = " Serialize and COBS encode a `T` to a `std::vec::Vec<u8>`."] # [doc = ""] # [doc = " The terminating sentinel `0x00` byte is included in the output."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard2::to_stdvec_cobs;"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_stdvec_cobs(&true).unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x02, 0x01, 0x00]);"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_stdvec_cobs(\"Hi!\").unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);"] # [doc = " ```"] # [cfg (feature = "use-std")] # [cfg_attr (docsrs , doc (cfg (feature = "use-std")))] # [inline] pub fn to_stdvec_cobs < T > (value : & T) -> Result < std :: vec :: Vec < u8 > > where T : Serialize + ? Sized , { to_allocvec_cobs (value) }
};
}
