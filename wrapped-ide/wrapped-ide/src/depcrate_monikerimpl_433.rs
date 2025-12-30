// Generated macro for impl_433 (impl)
macro_rules! Depcrate_monikerimpl_433 {
() => {
// Module: crate::moniker
// Provides: {"impl_433"}
// Dependencies: {}
impl fmt :: Display for MonikerIdentifier { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& self . crate_name) ? ; f . write_fmt (format_args ! ("::{}" , self . description . iter () . map (| x | & x . name) . join ("::"))) } }
};
}
