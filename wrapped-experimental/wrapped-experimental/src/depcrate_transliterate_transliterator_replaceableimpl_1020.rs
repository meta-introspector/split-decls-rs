// Generated macro for impl_1020 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1020 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1020"}
// Dependencies: {}
impl < 'a > Hide < 'a > { fn new (raw : & 'a mut Vec < u8 >) -> Self { Self { raw , hide_pre_len : 0 , hide_post_len : 0 , } } fn splice (& mut self , range : Range < usize > , replace_with : impl IntoIterator < Item = u8 >) { let adjusted_range = range . start + self . hide_pre_len .. range . end + self . hide_pre_len ; self . raw . splice (adjusted_range , replace_with) ; } fn child (& mut self) -> Hide < '_ > { Hide { raw : self . raw , hide_pre_len : self . hide_pre_len , hide_post_len : self . hide_post_len , } } # [doc = " Borrows into a child `Hide` with its visible part restricted to the given range."] fn tighten (& mut self , visible_range : Range < usize >) -> Hide < '_ > { debug_assert ! (visible_range . start <= self . len ()) ; debug_assert ! (visible_range . end <= self . len ()) ; let hide_pre_len = self . hide_pre_len + visible_range . start ; let hide_post_len = self . hide_post_len + (self . len () - visible_range . end) ; Hide { raw : self . raw , hide_pre_len , hide_post_len , } } fn hidden_prefix (& self) -> & [u8] { & self . raw [.. self . hide_pre_len] } fn hidden_suffix (& self) -> & [u8] { & self . raw [self . raw . len () - self . hide_post_len ..] } }
};
}
