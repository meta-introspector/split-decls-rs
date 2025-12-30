// Generated macro for impl_316 (impl)
macro_rules! Depcrate_traitsimpl_316 {
() => {
// Module: crate::traits
// Provides: {"impl_316"}
// Dependencies: {}
impl < I , C > CompactStringExt for C where I : AsRef < str > , C : IntoIterator < Item = I > , { fn concat_compact (self) -> CompactString { self . into_iter () . fold (CompactString :: const_new ("") , | mut s , item | { s . push_str (item . as_ref ()) ; s }) } fn join_compact < S : AsRef < str > > (self , separator : S) -> CompactString { let mut compact_string = CompactString :: const_new ("") ; let mut iter = self . into_iter () . peekable () ; let sep = separator . as_ref () ; while let Some (item) = iter . next () { compact_string . push_str (item . as_ref ()) ; if iter . peek () . is_some () { compact_string . push_str (sep) ; } } compact_string } }
};
}
