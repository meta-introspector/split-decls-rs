// Generated macro for to_eio (function)
macro_rules! Depcrate_serto_eio {
() => {
// Module: crate::ser
// Provides: {"to_eio"}
// Dependencies: {}
# [doc = " Serialize a `T` to an [`embedded_io Write`](crate::eio::Write),"] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard::to_eio;"] # [doc = " let mut buf: [u8; 32] = [0; 32];"] # [doc = " let mut writer: &mut [u8] = &mut buf;"] # [doc = ""] # [doc = " let ser = to_eio(&true, &mut writer).unwrap();"] # [doc = " to_eio(\"Hi!\", ser).unwrap();"] # [doc = " assert_eq!(&buf[0..5], &[0x01, 0x03, b'H', b'i', b'!']);"] # [doc = " ```"] # [cfg (any (feature = "embedded-io-04" , feature = "embedded-io-06"))] pub fn to_eio < T , W > (value : & T , writer : W) -> Result < W > where T : Serialize + ? Sized , W : crate :: eio :: Write , { serialize_with_flavor :: < T , _ , _ > (value , flavors :: eio :: WriteFlavor :: new (writer)) }
};
}
