// Generated macro for tests (module)
macro_rules! Depcrate_thread_safetytests {
() => {
// Module: crate::thread_safety
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [allow (unused_imports)] use crate :: * ; # [cfg (feature = "CTFont")] static_assertions :: assert_not_impl_any ! (CTFont : Send , Sync) ; # [cfg (feature = "CTFontCollection")] static_assertions :: assert_not_impl_any ! (CTFontCollection : Send , Sync) ; # [cfg (feature = "CTFontDescriptor")] static_assertions :: assert_not_impl_any ! (CTFontDescriptor : Send , Sync) ; # [cfg (feature = "CTGlyphInfo")] static_assertions :: assert_not_impl_any ! (CTGlyphInfo : Send , Sync) ; }
};
}
