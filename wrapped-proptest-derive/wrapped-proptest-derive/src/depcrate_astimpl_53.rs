// Generated macro for impl_53 (impl)
macro_rules! Depcrate_astimpl_53 {
() => {
// Module: crate::ast
// Provides: {"impl_53"}
// Dependencies: {}
impl ToTokens for Ctor { fn to_tokens (& self , tokens : & mut TokenStream) { use self :: Ctor :: * ; match self { Filter (ctor , filter) => quote_append ! (tokens , _proptest :: strategy :: Strategy :: prop_filter (# ctor , stringify ! (# filter) , # filter)) , Extract (ctor , to , from) => quote_append ! (tokens , { let # to = # from ; # ctor }) , Arbitrary (ty , fv , span) => { tokens . append_all (if let Some (fv) = fv { let args = param (* fv) ; quote_spanned ! (* span => _proptest :: arbitrary :: any_with ::<# ty > (# args)) } else { quote_spanned ! (* span => _proptest :: arbitrary :: any ::<# ty > ()) }) } Regex (ty , regex) => quote_append ! (tokens , <# ty as _proptest :: string :: StrategyFromRegex >:: from_regex (# regex)) , Existential (expr) => quote_append ! (tokens , _proptest :: strategy :: Strategy :: boxed (# expr)) , Value (expr) => quote_append ! (tokens , (|| # expr) as fn () -> _) , ValueExistential (expr) => quote_append ! (tokens , _proptest :: strategy :: Strategy :: boxed (_proptest :: strategy :: LazyJust :: new (move || # expr))) , Map (ctors , closure) => map_ctor_to_tokens (tokens , & ctors , closure) , # [cfg (not (feature = "boxed_union"))] Union (ctors) => union_ctor_to_tokens (tokens , ctors) , # [cfg (feature = "boxed_union")] Union (ctors) => union_ctor_to_tokens_boxed (tokens , ctors) , } } }
};
}
