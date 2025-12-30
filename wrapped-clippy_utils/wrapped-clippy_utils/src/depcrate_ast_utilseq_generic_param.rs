// Generated macro for eq_generic_param (function)
macro_rules! Depcrate_ast_utilseq_generic_param {
() => {
// Module: crate::ast_utils
// Provides: {"eq_generic_param"}
// Dependencies: {}
pub fn eq_generic_param (l : & GenericParam , r : & GenericParam) -> bool { use GenericParamKind :: * ; l . is_placeholder == r . is_placeholder && eq_id (l . ident , r . ident) && over (& l . bounds , & r . bounds , eq_generic_bound) && match (& l . kind , & r . kind) { (Lifetime , Lifetime) => true , (Type { default : l } , Type { default : r }) => both (l . as_ref () , r . as_ref () , | l , r | eq_ty (l , r)) , (Const { ty : lt , default : ld , span : _ , } , Const { ty : rt , default : rd , span : _ , } ,) => eq_ty (lt , rt) && both (ld . as_ref () , rd . as_ref () , eq_anon_const) , _ => false , } && over (& l . attrs , & r . attrs , eq_attr) }
};
}
