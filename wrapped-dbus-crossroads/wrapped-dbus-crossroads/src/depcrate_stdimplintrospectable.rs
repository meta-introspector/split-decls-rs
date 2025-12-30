// Generated macro for introspectable (function)
macro_rules! Depcrate_stdimplintrospectable {
() => {
// Module: crate::stdimpl
// Provides: {"introspectable"}
// Dependencies: {}
pub fn introspectable (cr : & mut Crossroads) -> IfaceToken < () > { cr . register ("org.freedesktop.DBus.Introspectable" , | b | { b . method_with_cr ("Introspect" , () , ("xml_data" ,) , | ctx , cr , _ : () | { Ok ((introspect (cr , ctx . path ()) ,)) }) ; }) }
};
}
