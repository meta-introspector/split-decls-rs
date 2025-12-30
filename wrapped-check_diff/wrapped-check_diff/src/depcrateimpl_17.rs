// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Display for Diff { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let patch = diffy :: create_patch (self . src_format . as_str () , self . feature_format . as_str ()) ; write ! (f , "{}" , patch) } }
};
}
