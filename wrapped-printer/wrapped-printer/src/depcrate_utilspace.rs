// Generated macro for Space (struct)
macro_rules! Depcrate_utilSpace {
() => {
// Module: crate::util
// Provides: {"Space"}
// Dependencies: {}
struct Space < M : Matcher > { # [doc = " The place to store capture locations."] caps : M :: Captures , # [doc = " The place to write a replacement to."] dst : Vec < u8 > , # [doc = " The place to store match offsets in terms of `dst`."] matches : Vec < Match > , }
};
}
