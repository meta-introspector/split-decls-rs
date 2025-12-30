// Generated macro for impl_81 (impl)
macro_rules! Depcrate_errorimpl_81 {
() => {
// Module: crate::error
// Provides: {"impl_81"}
// Dependencies: {}
impl fmt :: Display for ConfigError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ConfigError :: Frozen => write ! (f , "configuration is frozen") , ConfigError :: PathParse { ref cause } => write ! (f , "{cause}") , ConfigError :: Message (ref s) => write ! (f , "{s}") , ConfigError :: Foreign (ref cause) => write ! (f , "{cause}") , ConfigError :: NotFound (ref key) => { write ! (f , "missing configuration field {key:?}") } ConfigError :: Type { ref origin , ref unexpected , expected , ref key , } => { write ! (f , "invalid type: {unexpected}, expected {expected}") ? ; if let Some (ref key) = * key { write ! (f , " for key `{key}`") ? ; } if let Some (ref origin) = * origin { write ! (f , " in {origin}") ? ; } Ok (()) } ConfigError :: At { ref error , ref origin , ref key , } => { write ! (f , "{error}") ? ; if let Some (ref key) = * key { write ! (f , " for key `{key}`") ? ; } if let Some (ref origin) = * origin { write ! (f , " in {origin}") ? ; } Ok (()) } ConfigError :: FileParse { ref cause , ref uri } => { write ! (f , "{cause}") ? ; if let Some (ref uri) = * uri { write ! (f , " in {uri}") ? ; } Ok (()) } } } }
};
}
