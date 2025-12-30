// Generated macro for impl_191 (impl)
macro_rules! Depcrate_manifestimpl_191 {
() => {
// Module: crate::manifest
// Provides: {"impl_191"}
// Dependencies: {}
impl fmt :: Display for TomlTrimPaths { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TomlTrimPaths :: All => write ! (f , "all") , TomlTrimPaths :: Values (v) if v . is_empty () => write ! (f , "none") , TomlTrimPaths :: Values (v) => { let mut iter = v . iter () ; if let Some (value) = iter . next () { write ! (f , "{value}") ? ; } for value in iter { write ! (f , ",{value}") ? ; } Ok (()) } } } }
};
}
