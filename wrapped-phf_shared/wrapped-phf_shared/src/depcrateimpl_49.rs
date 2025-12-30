// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
# [cfg (feature = "unicase")] impl < S > FmtConst for unicase :: Ascii < S > where S : AsRef < str > , { fn fmt_const (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Ascii::new(") ? ; self . as_ref () . fmt_const (f) ? ; f . write_str (")") } }
};
}
