// Generated macro for impl_402 (impl)
macro_rules! Depcrate_redactionimpl_402 {
() => {
// Module: crate::redaction
// Provides: {"impl_402"}
// Dependencies: {}
impl fmt :: Display for ContentPath < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for item in self . 0 . iter () { write ! (f , ".") ? ; match * item { PathItem :: Content (ref ctx) => { if let Some (s) = ctx . as_str () { write ! (f , "{s}") ? ; } else { write ! (f , "<content>") ? ; } } PathItem :: Field (name) => write ! (f , "{name}") ? , PathItem :: Index (idx , _) => write ! (f , "{idx}") ? , } } Ok (()) } }
};
}
