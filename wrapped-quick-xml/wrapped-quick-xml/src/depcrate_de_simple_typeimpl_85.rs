// Generated macro for impl_85 (impl)
macro_rules! Depcrate_de_simple_typeimpl_85 {
() => {
// Module: crate::de::simple_type
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'de , 'a > Content < 'de , 'a > { # [doc = " Returns string representation of the content"] fn as_str (& self) -> & str { match self { Content :: Input (s) => s , Content :: Slice (s) => s , Content :: Owned (s , offset) => s . split_at (* offset) . 1 , } } }
};
}
