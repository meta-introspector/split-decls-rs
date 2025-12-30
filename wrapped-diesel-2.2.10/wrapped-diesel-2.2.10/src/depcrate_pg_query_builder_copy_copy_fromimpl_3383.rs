// Generated macro for impl_3383 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromimpl_3383 {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"impl_3383"}
// Dependencies: {}
impl QueryFragment < Pg > for CopyFromOptions { fn walk_ast < 'b > (& 'b self , mut pass : crate :: query_builder :: AstPass < '_ , 'b , Pg > ,) -> crate :: QueryResult < () > { if self . any_set () { let mut comma = "" ; pass . push_sql (" WITH (") ; self . common . walk_ast (pass . reborrow () , & mut comma) ; if let Some (ref default) = self . default { pass . push_sql (comma) ; comma = ", " ; pass . push_sql ("DEFAULT '") ; pass . push_sql (default) ; pass . push_sql ("'") ; } if let Some (ref header) = self . header { pass . push_sql (comma) ; pass . push_sql ("HEADER ") ; match header { CopyHeader :: Set (true) => pass . push_sql ("1") , CopyHeader :: Set (false) => pass . push_sql ("0") , CopyHeader :: Match => pass . push_sql ("MATCH") , } } pass . push_sql (")") ; } Ok (()) } }
};
}
