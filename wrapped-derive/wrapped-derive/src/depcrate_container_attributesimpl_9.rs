// Generated macro for impl_9 (impl)
macro_rules! Depcrate_container_attributesimpl_9 {
() => {
// Module: crate::container_attributes
// Provides: {"impl_9"}
// Dependencies: {}
impl ContainerAttributes { pub fn from_derive_input (derive_input : & DeriveInput) -> Result < Self , Error > { let mut bounds = None ; for attr in & derive_input . attrs { if ! attr . path () . is_ident (ARBITRARY_ATTRIBUTE_NAME) { continue ; } let meta_list = match attr . meta { Meta :: List (ref l) => l , _ => { return Err (Error :: new_spanned (attr , format ! ("invalid `{}` attribute. expected list" , ARBITRARY_ATTRIBUTE_NAME) ,)) } } ; for nested_meta in meta_list . parse_args_with (Punctuated :: < Meta , Token ! [,] > :: parse_terminated) ? { match nested_meta { Meta :: NameValue (MetaNameValue { path , value : Expr :: Lit (ExprLit { lit : Lit :: Str (bound_str_lit) , .. }) , .. }) if path . is_ident ("bound") => { bounds . get_or_insert_with (Vec :: new) . push (bound_str_lit . parse_with (Punctuated :: parse_terminated) ?) ; } _ => { return Err (Error :: new_spanned (attr , format ! ("invalid `{}` attribute. expected `bound = \"..\"`" , ARBITRARY_ATTRIBUTE_NAME ,) ,)) } } } } Ok (Self { bounds }) } }
};
}
