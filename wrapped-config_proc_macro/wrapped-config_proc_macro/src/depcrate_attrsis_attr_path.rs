// Generated macro for is_attr_path (function)
macro_rules! Depcrate_attrsis_attr_path {
() => {
// Module: crate::attrs
// Provides: {"is_attr_path"}
// Dependencies: {}
fn is_attr_path (attr : & syn :: Attribute , name : & str) -> bool { match & attr . meta { syn :: Meta :: Path (path) if path . is_ident (name) => true , _ => false , } }
};
}
