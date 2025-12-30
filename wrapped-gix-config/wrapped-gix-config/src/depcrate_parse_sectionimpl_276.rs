// Generated macro for impl_276 (impl)
macro_rules! Depcrate_parse_sectionimpl_276 {
() => {
// Module: crate::parse::section
// Provides: {"impl_276"}
// Dependencies: {}
impl Display for Section < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . header) ? ; for event in & self . events { event . fmt (f) ? ; } Ok (()) } }
};
}
