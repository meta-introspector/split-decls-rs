// Generated macro for impl_2060 (impl)
macro_rules! Depcrate_wtf8impl_2060 {
() => {
// Module: crate::wtf8
// Provides: {"impl_2060"}
// Dependencies: {}
# [doc = " Append code points from an iterator to the string."] # [doc = ""] # [doc = " This replaces surrogate code point pairs with supplementary code points,"] # [doc = " like concatenating ill-formed UTF-16 strings effectively would."] impl Extend < CodePoint > for Wtf8Buf { fn extend < T : IntoIterator < Item = CodePoint > > (& mut self , iter : T) { let iterator = iter . into_iter () ; let (low , _high) = iterator . size_hint () ; self . bytes . reserve (low) ; iterator . for_each (move | code_point | self . push (code_point)) ; } # [inline] fn extend_one (& mut self , code_point : CodePoint) { self . push (code_point) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . bytes . reserve (additional) ; } }
};
}
