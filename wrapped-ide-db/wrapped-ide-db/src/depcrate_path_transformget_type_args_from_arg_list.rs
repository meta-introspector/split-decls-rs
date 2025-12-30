// Generated macro for get_type_args_from_arg_list (function)
macro_rules! Depcrate_path_transformget_type_args_from_arg_list {
() => {
// Module: crate::path_transform
// Provides: {"get_type_args_from_arg_list"}
// Dependencies: {}
fn get_type_args_from_arg_list (generic_arg_list : ast :: GenericArgList) -> Option < AstSubsts > { let mut result = AstSubsts :: default () ; generic_arg_list . generic_args () . for_each (| generic_arg | match generic_arg { ast :: GenericArg :: TypeArg (type_arg) => { result . types_and_consts . push (TypeOrConst :: Either (type_arg)) } ast :: GenericArg :: ConstArg (const_arg) => { result . types_and_consts . push (TypeOrConst :: Const (const_arg)) ; } ast :: GenericArg :: LifetimeArg (l_arg) => result . lifetimes . push (l_arg) , _ => () , }) ; Some (result) }
};
}
