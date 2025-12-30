// Generated macro for map_range (function)
macro_rules! Depcrate_sourcemap_range {
() => {
// Module: crate::source
// Provides: {"map_range"}
// Dependencies: {}
# [expect (clippy :: cast_possible_truncation)] fn map_range (sm : & SourceMap , sp : Range < BytePos > , f : impl for < 'a > FnOnce (& 'a SourceFile , & 'a str , Range < usize >) -> Option < Range < usize > > ,) -> Option < Range < BytePos > > { if let Some (src) = get_source_range (sm , sp . clone ()) && let Some (text) = & src . sf . src && let Some (range) = f (& src . sf , text , src . range . clone ()) { debug_assert ! (range . start <= text . len () && range . end <= text . len () , "Range `{range:?}` is outside the source file (file `{}`, length `{}`)" , src . sf . name . display (FileNameDisplayPreference :: Local) , text . len () ,) ; debug_assert ! (range . start <= range . end , "Range `{range:?}` has overlapping bounds") ; let dstart = (range . start as u32) . wrapping_sub (src . range . start as u32) ; let dend = (range . end as u32) . wrapping_sub (src . range . start as u32) ; Some (BytePos (sp . start . 0 . wrapping_add (dstart)) .. BytePos (sp . start . 0 . wrapping_add (dend))) } else { None } }
};
}
