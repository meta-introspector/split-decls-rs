// Generated macro for copy_target_for_columns (macro)
macro_rules! Depcrate_pg_query_builder_copycopy_target_for_columns {
() => {
// Module: crate::pg::query_builder::copy
// Provides: {"copy_target_for_columns"}
// Dependencies: {}
macro_rules ! copy_target_for_columns { ($ ($ Tuple : tt { $ (($ idx : tt) -> $ T : ident , $ ST : ident , $ TT : ident ,) + }) +) => { $ (impl < T , $ ($ ST ,) *> CopyTarget for ($ ($ ST ,) *) where $ ($ ST : Column < Table = T > + Default ,) * ($ (<$ ST as Expression >:: SqlType ,) *) : SqlType , T : Table + StaticQueryFragment , T :: Component : QueryFragment < Pg >, Self : ColumnList , { type Table = T ; type SqlType = crate :: dsl :: SqlTypeOf < Self >; fn walk_target (mut pass : crate :: query_builder :: AstPass <'_ , '_ , Pg >,) -> crate :: QueryResult < () > { T :: STATIC_COMPONENT . walk_ast (pass . reborrow ()) ?; pass . push_sql ("(") ; < Self as ColumnList >:: walk_ast (& ($ ($ ST :: default () ,) *) , pass . reborrow ()) ?; pass . push_sql (")") ; Ok (()) } }) * } }
};
}
