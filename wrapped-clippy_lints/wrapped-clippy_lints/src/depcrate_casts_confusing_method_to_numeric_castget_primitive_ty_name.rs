// Generated macro for get_primitive_ty_name (function)
macro_rules! Depcrate_casts_confusing_method_to_numeric_castget_primitive_ty_name {
() => {
// Module: crate::casts::confusing_method_to_numeric_cast
// Provides: {"get_primitive_ty_name"}
// Dependencies: {}
fn get_primitive_ty_name (ty : Ty < '_ >) -> Option < & 'static str > { match ty . kind () { ty :: Char => Some ("char") , ty :: Int (int) => Some (int . name_str ()) , ty :: Uint (uint) => Some (uint . name_str ()) , ty :: Float (float) => Some (float . name_str ()) , _ => None , } }
};
}
