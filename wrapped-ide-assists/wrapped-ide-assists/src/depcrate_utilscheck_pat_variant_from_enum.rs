// Generated macro for check_pat_variant_from_enum (function)
macro_rules! Depcrate_utilscheck_pat_variant_from_enum {
() => {
// Module: crate::utils
// Provides: {"check_pat_variant_from_enum"}
// Dependencies: {}
fn check_pat_variant_from_enum (ctx : & AssistContext < '_ > , pat : & ast :: Pat) -> bool { ctx . sema . type_of_pat (pat) . is_none_or (| ty : hir :: TypeInfo < '_ > | { ty . adjusted () . as_adt () . is_some_and (| adt | matches ! (adt , hir :: Adt :: Enum (_))) }) }
};
}
