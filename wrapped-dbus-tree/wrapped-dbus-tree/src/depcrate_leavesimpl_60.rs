// Generated macro for impl_60 (impl)
macro_rules! Depcrate_leavesimpl_60 {
() => {
// Module: crate::leaves
// Provides: {"impl_60"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > Introspect for Method < M , D > { fn xml_name (& self) -> & 'static str { "method" } fn xml_params (& self) -> String { String :: new () } fn xml_contents (& self) -> String { format ! ("{}{}{}" , introspect_args (& self . i_args , "      " , " direction=\"in\"") , introspect_args (& self . o_args , "      " , " direction=\"out\"") , self . anns . introspect ("      ")) } }
};
}
