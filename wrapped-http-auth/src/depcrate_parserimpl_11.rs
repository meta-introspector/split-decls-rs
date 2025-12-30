// Generated macro for impl_11 (impl)
macro_rules! Depcrate_parserimpl_11 {
() => {
// Module: crate::parser
// Provides: {"impl_11"}
// Dependencies: {}
impl Display for Error < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{} at byte {}: {:?}" , self . error , self . pos , format_args ! ("{}(HERE-->){}" , & self . input [.. self . pos] , & self . input [self . pos ..]) ,) } }
};
}
