// Generated macro for impl_2059 (impl)
macro_rules! Depcrate_wtf8impl_2059 {
() => {
// Module: crate::wtf8
// Provides: {"impl_2059"}
// Dependencies: {}
# [doc = " Creates a new WTF-8 string from an iterator of code points."] # [doc = ""] # [doc = " This replaces surrogate code point pairs with supplementary code points,"] # [doc = " like concatenating ill-formed UTF-16 strings effectively would."] impl FromIterator < CodePoint > for Wtf8Buf { fn from_iter < T : IntoIterator < Item = CodePoint > > (iter : T) -> Wtf8Buf { let mut string = Wtf8Buf :: new () ; string . extend (iter) ; string } }
};
}
