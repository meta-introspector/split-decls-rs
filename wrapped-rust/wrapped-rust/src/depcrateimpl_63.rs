// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl std :: fmt :: Display for WithOption { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { WithOption :: Path (p) => f . write_fmt (format_args ! ("\"{p}\"")) , WithOption :: Generate => f . write_str ("generate") , } } }
};
}
