// Generated macro for introspect_item (function)
macro_rules! Depcrate_parseintrospect_item {
() => {
// Module: crate::parse
// Provides: {"introspect_item"}
// Dependencies: {}
fn introspect_item (item : & AstItem , lookup : & Lookup) -> types :: Node { let features = introspect_features (& item . features) ; match & item . ast . data { Data :: Enum (data) => types :: Node { ident : item . ast . ident . to_string () , features , data : types :: Data :: Enum (introspect_enum (data , lookup)) , exhaustive : ! (is_non_exhaustive (& item . ast . attrs) || data . variants . iter () . any (| v | is_doc_hidden (& v . attrs))) , } , Data :: Struct (data) => types :: Node { ident : item . ast . ident . to_string () , features , data : { if data . fields . iter () . all (| f | is_pub (& f . vis)) { types :: Data :: Struct (introspect_struct (data , lookup)) } else { types :: Data :: Private } } , exhaustive : ! is_non_exhaustive (& item . ast . attrs) , } , Data :: Union (..) => panic ! ("union not supported") , } }
};
}
