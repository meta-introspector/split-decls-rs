// Generated macro for impl_455 (impl)
macro_rules! Depcrate_from_metaimpl_455 {
() => {
// Module: crate::from_meta
// Provides: {"impl_455"}
// Dependencies: {}
impl FromMeta for Vec < syn :: WherePredicate > { fn from_string (value : & str) -> Result < Self > { syn :: WhereClause :: from_string (& format ! ("where {}" , value)) . map (| c | c . predicates . into_iter () . collect ()) } fn from_value (value : & Lit) -> Result < Self > { if let syn :: Lit :: Str (s) = value { syn :: WhereClause :: from_value (& syn :: Lit :: Str (syn :: LitStr :: new (& format ! ("where {}" , s . value ()) , value . span () ,))) . map (| c | c . predicates . into_iter () . collect ()) } else { Err (Error :: unexpected_lit_type (value)) } } }
};
}
