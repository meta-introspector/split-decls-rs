// Generated macro for impl_97 (impl)
macro_rules! Depcrate_objectpathimpl_97 {
() => {
// Module: crate::objectpath
// Provides: {"impl_97"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > Introspect for Interface < M , D > { fn xml_name (& self) -> & 'static str { "interface" } fn xml_params (& self) -> String { String :: new () } fn xml_contents (& self) -> String { format ! ("{}{}{}{}" , introspect_map (& self . methods , "    ") , introspect_map (& self . properties , "    ") , introspect_map (& self . signals , "    ") , self . anns . introspect ("    ")) } }
};
}
