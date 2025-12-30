// Generated macro for impl_format_escaped_tuple (macro)
macro_rules! Depcrate_svgimpl_format_escaped_tuple {
() => {
// Module: crate::svg
// Provides: {"impl_format_escaped_tuple"}
// Dependencies: {}
macro_rules ! impl_format_escaped_tuple { ($ ($ idx : tt $ t : tt) ,+) => { impl <$ ($ t ,) +> FormatEscaped for ($ ($ t ,) +) where $ ($ t : FormatEscaped ,) + { fn format_escaped (buf : & mut String , tup : Self) { $ (FormatEscaped :: format_escaped (buf , tup .$ idx) ;) + } } } ; }
};
}
