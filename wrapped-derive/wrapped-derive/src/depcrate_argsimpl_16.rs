// Generated macro for impl_16 (impl)
macro_rules! Depcrate_argsimpl_16 {
() => {
// Module: crate::args
// Provides: {"impl_16"}
// Dependencies: {}
impl FromMeta for PathList { fn from_list (items : & [NestedMeta]) -> darling :: Result < Self > { let mut res = Vec :: new () ; for item in items { match item { NestedMeta :: Meta (Meta :: Path (p)) => res . push (p . clone ()) , NestedMeta :: Lit (Lit :: Str (s)) => { res . push (syn :: parse_str :: < Path > (& s . value ()) . map_err (| _ | { darling :: Error :: custom (format ! ("Invalid path: {}" , s . value ())) }) ?) } _ => return Err (darling :: Error :: custom ("Invalid path list")) , } } Ok (PathList (res)) } }
};
}
