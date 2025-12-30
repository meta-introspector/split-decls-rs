// Generated macro for define_config_type (function)
macro_rules! Depcrate_config_typedefine_config_type {
() => {
// Module: crate::config_type
// Provides: {"define_config_type"}
// Dependencies: {}
# [doc = " Defines `config_type` on enum or struct."] pub fn define_config_type (input : & syn :: Item) -> TokenStream { match input { syn :: Item :: Struct (st) => define_config_type_on_struct (st) , syn :: Item :: Enum (en) => define_config_type_on_enum (en) , _ => panic ! ("Expected enum or struct") , } . unwrap () }
};
}
