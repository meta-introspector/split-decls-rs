// Generated macro for external_bitflags_debug (macro)
macro_rules! Depcrateexternal_bitflags_debug {
() => {
// Module: crate
// Provides: {"external_bitflags_debug"}
// Dependencies: {}
# [macro_export] macro_rules ! external_bitflags_debug { ($ Name : ident) => { impl :: std :: fmt :: Debug for $ Name { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: fmt :: Result { :: bitflags :: parser :: to_writer (self , f) } } } ; }
};
}
