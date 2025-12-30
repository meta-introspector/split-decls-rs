// Generated macro for impl_36 (impl)
macro_rules! Depcrate_confimpl_36 {
() => {
// Module: crate::conf
// Provides: {"impl_36"}
// Dependencies: {}
impl ConfError { fn from_toml (file : & SourceFile , error : & toml :: de :: Error) -> Self { let span = error . span () . unwrap_or (0 .. file . source_len . 0 as usize) ; Self :: spanned (file , error . message () , None , span) } fn spanned (file : & SourceFile , message : impl Into < String > , suggestion : Option < Suggestion > , span : Range < usize > ,) -> Self { Self { message : message . into () , suggestion , span : span_from_toml_range (file , span) , } } }
};
}
