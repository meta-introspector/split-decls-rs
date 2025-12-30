// Generated macro for impl_5 (impl)
macro_rules! Depcrate_fmtimpl_5 {
() => {
// Module: crate::fmt
// Provides: {"impl_5"}
// Dependencies: {}
impl < T : embedded_io :: Write + ? Sized > core :: fmt :: Write for ToFmt < T > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { self . inner . write_all (s . as_bytes ()) . or (Err (core :: fmt :: Error)) } }
};
}
