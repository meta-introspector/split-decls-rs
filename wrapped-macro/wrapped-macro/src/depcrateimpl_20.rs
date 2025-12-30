// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl Config { fn expand (self) -> Result < TokenStream > { let mut files = Default :: default () ; let mut generator = self . opts . build () ; generator . generate (& self . resolve , self . world , & mut files) . map_err (| e | anyhow_to_syn (Span :: call_site () , e)) ? ; let (_ , src) = files . iter () . next () . unwrap () ; let mut src = std :: str :: from_utf8 (src) . unwrap () . to_string () ; if std :: env :: var ("WIT_BINDGEN_DEBUG") . is_ok () || self . debug { static INVOCATION : AtomicUsize = AtomicUsize :: new (0) ; let root = Path :: new (env ! ("DEBUG_OUTPUT_DIR")) ; let world_name = & self . resolve . worlds [self . world] . name ; let n = INVOCATION . fetch_add (1 , Relaxed) ; let path = root . join (format ! ("{world_name}{n}.rs")) ; let contents = match fmt (& src) { Ok (formatted) => formatted , Err (_) => src . clone () , } ; std :: fs :: write (& path , contents . as_bytes ()) . unwrap () ; src = format ! ("include!({path:?});") ; } let mut contents = src . parse :: < TokenStream > () . unwrap () ; for file in self . files . iter () { contents . extend (format ! ("const _: &[u8] = include_bytes!(r#\"{}\"#);\n" , file . display ()) . parse :: < TokenStream > () . unwrap () ,) ; } Ok (contents) } }
};
}
