// Generated macro for to_io (function)
macro_rules! Depcrate_serto_io {
() => {
// Module: crate::ser
// Provides: {"to_io"}
// Dependencies: {}
# [doc = " Serialize a `T` to a [`std::io::Write`],"] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard::to_io;"] # [doc = " let mut buf: [u8; 32] = [0; 32];"] # [doc = " let mut writer: &mut [u8] = &mut buf;"] # [doc = ""] # [doc = " let ser = to_io(&true, &mut writer).unwrap();"] # [doc = " to_io(\"Hi!\", ser).unwrap();"] # [doc = " assert_eq!(&buf[0..5], &[0x01, 0x03, b'H', b'i', b'!']);"] # [doc = " ```"] # [cfg (feature = "use-std")] pub fn to_io < T , W > (value : & T , writer : W) -> Result < W > where T : Serialize + ? Sized , W : std :: io :: Write , { serialize_with_flavor :: < T , _ , _ > (value , flavors :: io :: WriteFlavor :: new (writer)) }
};
}
