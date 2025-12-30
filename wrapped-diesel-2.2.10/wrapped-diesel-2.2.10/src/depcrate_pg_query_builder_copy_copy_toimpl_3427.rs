// Generated macro for impl_3427 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_toimpl_3427 {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"impl_3427"}
// Dependencies: {}
impl QueryFragment < Pg > for CopyToOptions { fn walk_ast < 'b > (& 'b self , mut pass : crate :: query_builder :: AstPass < '_ , 'b , Pg > ,) -> crate :: QueryResult < () > { if self . any_set () { let mut comma = "" ; pass . push_sql (" WITH (") ; self . common . walk_ast (pass . reborrow () , & mut comma) ; if let Some (header_is_set) = self . header { pass . push_sql (comma) ; pass . push_sql ("HEADER ") ; pass . push_sql (if header_is_set { "1" } else { "0" }) ; } pass . push_sql (")") ; } Ok (()) } }
};
}
