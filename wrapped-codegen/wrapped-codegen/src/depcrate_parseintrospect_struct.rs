// Generated macro for introspect_struct (function)
macro_rules! Depcrate_parseintrospect_struct {
() => {
// Module: crate::parse
// Provides: {"introspect_struct"}
// Dependencies: {}
fn introspect_struct (item : & DataStruct , lookup : & Lookup) -> types :: Fields { match & item . fields { Fields :: Named (fields) => fields . named . iter () . map (| field | { (field . ident . as_ref () . unwrap () . to_string () , introspect_type (& field . ty , lookup) ,) }) . collect () , Fields :: Unit => IndexMap :: new () , Fields :: Unnamed (_) => panic ! ("struct representation not supported") , } }
};
}
