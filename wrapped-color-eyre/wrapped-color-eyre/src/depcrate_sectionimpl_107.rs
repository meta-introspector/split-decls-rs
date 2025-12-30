// Generated macro for impl_107 (impl)
macro_rules! Depcrate_sectionimpl_107 {
() => {
// Module: crate::section
// Provides: {"impl_107"}
// Dependencies: {}
impl < H , B > fmt :: Display for IndentedSection < H , B > where H : Display + Send + Sync + 'static , B : Display + Send + Sync + 'static , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use std :: fmt :: Write ; let mut headered = f . header (& self . header) ; let headered = headered . ready () ; let mut headered = headered . header ("\n") ; let mut headered = headered . ready () ; let mut indented = indenter :: indented (& mut headered) . with_format (indenter :: Format :: Uniform { indentation : "   " }) ; write ! (& mut indented , "{}" , self . body) ? ; Ok (()) } }
};
}
