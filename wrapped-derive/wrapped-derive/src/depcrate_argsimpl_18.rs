// Generated macro for impl_18 (impl)
macro_rules! Depcrate_argsimpl_18 {
() => {
// Module: crate::args
// Provides: {"impl_18"}
// Dependencies: {}
impl FromMeta for GenericParamList { fn from_list (items : & [NestedMeta]) -> darling :: Result < Self > { let mut res = Vec :: new () ; for item in items { match item { NestedMeta :: Lit (Lit :: Str (s)) => { res . push (syn :: parse_str :: < GenericParam > (& s . value ()) . map_err (| _ | { darling :: Error :: custom (format ! ("Invalid GenericParam: {}" , s . value ())) }) ?) } _ => return Err (darling :: Error :: custom ("Invalid GenericParamList")) , } } Ok (GenericParamList (res)) } }
};
}
