// Generated macro for impl_302 (impl)
macro_rules! Depcrate_configimpl_302 {
() => {
// Module: crate::config
// Provides: {"impl_302"}
// Dependencies: {}
impl fmt :: Display for ConfigErrors { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let errors = self . 0 . iter () . format_with ("\n" , | inner , f | { match & * * inner { ConfigErrorInner :: Json { config_key : key , error : e } => { f (key) ? ; f (& ": ") ? ; f (e) } ConfigErrorInner :: Toml { config_key : key , error : e } => { f (key) ? ; f (& ": ") ? ; f (e) } ConfigErrorInner :: ParseError { reason } => f (reason) , } ? ; f (& ";") }) ; write ! (f , "invalid config value{}:\n{}" , if self . 0 . len () == 1 { "" } else { "s" } , errors) } }
};
}
