// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl std :: fmt :: Display for AddrTupleFmt { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { let AddrTupleFmt (src , dst) = & self ; if src . ip () . is_unspecified () || dst . ip () . is_unspecified () { return Ok (()) ; } f . write_fmt (format_args ! ("src:{src} dst:{dst}")) } }
};
}
