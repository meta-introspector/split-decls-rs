// Generated macro for StringInner (struct)
macro_rules! Depcrate_stringStringInner {
() => {
// Module: crate::string
// Provides: {"StringInner"}
// Dependencies: {}
# [doc = " Base struct for [`String`] and [`StringView`], generic over the [`StringStorage`]."] # [doc = ""] # [doc = " In most cases you should use [`String`] or [`StringView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] # [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "S: Zeroize"))] pub struct StringInner < LenT : LenType , S : StringStorage + ? Sized > { vec : VecInner < u8 , LenT , S > , }
};
}
