// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl std :: fmt :: Display for CompositeError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Error(s) rewriting input: {}" , self . 0 . iter () . map (| e | format ! ("{e:?}")) . collect ::< String > ()) } }
};
}
