// Generated macro for impl_64 (impl)
macro_rules! Depcrate_leavesimpl_64 {
() => {
// Module: crate::leaves
// Provides: {"impl_64"}
// Dependencies: {}
impl < D : DataType > Introspect for Signal < D > { fn xml_name (& self) -> & 'static str { "signal" } fn xml_params (& self) -> String { String :: new () } fn xml_contents (& self) -> String { format ! ("{}{}" , introspect_args (& self . arguments , "      " , "") , self . anns . introspect ("      ")) } }
};
}
