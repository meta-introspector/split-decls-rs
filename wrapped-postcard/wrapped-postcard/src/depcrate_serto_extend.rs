// Generated macro for to_extend (function)
macro_rules! Depcrate_serto_extend {
() => {
// Module: crate::ser
// Provides: {"to_extend"}
// Dependencies: {}
# [doc = " Serialize a `T` to a [`core::iter::Extend`],"] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard::to_extend;"] # [doc = " let mut vec = Vec::new();"] # [doc = ""] # [doc = " let ser = to_extend(&true, vec).unwrap();"] # [doc = " let vec = to_extend(\"Hi!\", ser).unwrap();"] # [doc = " assert_eq!(&vec[0..5], &[0x01, 0x03, b'H', b'i', b'!']);"] # [doc = " ```"] pub fn to_extend < T , W > (value : & T , writer : W) -> Result < W > where T : Serialize + ? Sized , W : core :: iter :: Extend < u8 > , { serialize_with_flavor :: < T , _ , _ > (value , flavors :: ExtendFlavor :: new (writer)) }
};
}
