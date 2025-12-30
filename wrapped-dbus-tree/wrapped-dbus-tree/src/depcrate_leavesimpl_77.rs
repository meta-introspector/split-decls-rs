// Generated macro for impl_77 (impl)
macro_rules! Depcrate_leavesimpl_77 {
() => {
// Module: crate::leaves
// Provides: {"impl_77"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > Introspect for Property < M , D > { fn xml_name (& self) -> & 'static str { "property" } fn xml_params (& self) -> String { format ! (" type=\"{}\" access=\"{}\"" , self . sig , self . rw . introspect ()) } fn xml_contents (& self) -> String { let s = match self . emits { EmitsChangedSignal :: True => return self . anns . introspect ("      ") , EmitsChangedSignal :: False => "false" , EmitsChangedSignal :: Const => "const" , EmitsChangedSignal :: Invalidates => "invalidates" , } ; let mut tempanns = self . anns . clone () ; tempanns . insert ("org.freedesktop.DBus.Property.EmitsChangedSignal" , s) ; tempanns . introspect ("      ") } }
};
}
