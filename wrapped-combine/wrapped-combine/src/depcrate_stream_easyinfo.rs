// Generated macro for Info (enum)
macro_rules! Depcrate_stream_easyInfo {
() => {
// Module: crate::stream::easy
// Provides: {"Info"}
// Dependencies: {}
# [doc = " Enum holding error information. Variants are defined for `Stream::Token` and `Stream::Range` as"] # [doc = " well as string variants holding easy descriptions."] # [doc = ""] # [doc = " As there is implementations of `From` for `String` and `&'static str` the"] # [doc = " constructor need not be used directly as calling `msg.into()` should turn a message into the"] # [doc = " correct `Info` variant."] # [derive (Clone , Debug)] pub enum Info < T , R > { Token (T) , Range (R) , Owned (String) , Static (& 'static str) , }
};
}
