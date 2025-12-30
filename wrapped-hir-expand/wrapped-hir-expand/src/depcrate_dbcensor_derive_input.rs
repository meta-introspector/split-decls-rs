// Generated macro for censor_derive_input (function)
macro_rules! Depcrate_dbcensor_derive_input {
() => {
// Module: crate::db
// Provides: {"censor_derive_input"}
// Dependencies: {}
# [doc = " Derives expect all `#[derive(..)]` invocations up to (and including) the currently invoked one to be stripped"] fn censor_derive_input (derive_attr_index : AttrId , node : & ast :: Adt) -> FxHashSet < SyntaxElement > { cov_mark :: hit ! (derive_censoring) ; collect_attrs (node) . take (derive_attr_index . ast_index () + 1) . filter_map (| (_ , attr) | Either :: left (attr)) . filter (| attr | attr . simple_name () . as_deref () == Some ("derive")) . map (| it | it . syntax () . clone () . into ()) . collect () }
};
}
