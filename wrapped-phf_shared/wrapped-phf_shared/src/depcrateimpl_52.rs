// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "uncased")] impl FmtConst for uncased :: UncasedStr { fn fmt_const (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("UncasedStr::new(") ? ; self . as_str () . fmt_const (f) ? ; f . write_str (")") } }
};
}
