// Generated macro for to_stdvec (function)
macro_rules! Depcrate_serto_stdvec {
() => {
// Module: crate::ser
// Provides: {"to_stdvec"}
// Dependencies: {}
# [doc = " Serialize a `T` to a `std::vec::Vec<u8>`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard::to_stdvec;"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_stdvec(&true).unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x01]);"] # [doc = ""] # [doc = " let ser: Vec<u8> = to_stdvec(\"Hi!\").unwrap();"] # [doc = " assert_eq!(ser.as_slice(), &[0x03, b'H', b'i', b'!']);"] # [doc = " ```"] # [cfg (feature = "use-std")] # [cfg_attr (docsrs , doc (cfg (feature = "use-std")))] # [inline] pub fn to_stdvec < T > (value : & T) -> Result < std :: vec :: Vec < u8 > > where T : Serialize + ? Sized , { to_allocvec (value) }
};
}
