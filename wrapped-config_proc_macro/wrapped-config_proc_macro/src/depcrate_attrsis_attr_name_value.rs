// Generated macro for is_attr_name_value (function)
macro_rules! Depcrate_attrsis_attr_name_value {
() => {
// Module: crate::attrs
// Provides: {"is_attr_name_value"}
// Dependencies: {}
fn is_attr_name_value (attr : & syn :: Attribute , name : & str) -> bool { match & attr . meta { syn :: Meta :: NameValue (syn :: MetaNameValue { path , .. }) if path . is_ident (name) => true , _ => false , } }
};
}
