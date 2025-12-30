// Generated macro for impl_180 (impl)
macro_rules! Depcrate_stream_easyimpl_180 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_180"}
// Dependencies: {}
impl < T , R , P > fmt :: Display for Errors < T , R , P > where P : fmt :: Display , T : fmt :: Display , R : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "Parse error at {}" , self . position) ? ; Error :: fmt_errors (& self . errors , f) } }
};
}
