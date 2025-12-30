// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (feature = "unicase")] impl < S > FmtConst for unicase :: UniCase < S > where S : AsRef < str > , { fn fmt_const (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_ascii () { f . write_str ("UniCase::ascii(") ? ; } else { f . write_str ("UniCase::unicode(") ? ; } self . as_ref () . fmt_const (f) ? ; f . write_str (")") } }
};
}
