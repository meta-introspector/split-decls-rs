// Generated macro for impl_983 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_983 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_983"}
// Dependencies: {}
impl < T , V , QId , Op , Ret , const STATIC_QUERY_ID : bool > DebugQueryHelper < Yes > for DebugQuery < '_ , InsertStatement < T , BatchInsert < Vec < ValuesClause < V , T > > , T , QId , STATIC_QUERY_ID > , Op , Ret > , Sqlite , > where V : QueryFragment < Sqlite > , T : Copy + QuerySource , Op : Copy , Ret : Copy , for < 'b > InsertStatement < T , & 'b ValuesClause < V , T > , Op , Ret > : QueryFragment < Sqlite > , { fn fmt_debug (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut statements = vec ! [String :: from ("BEGIN")] ; for record in self . query . records . values . iter () { let stmt = InsertStatement :: new (self . query . target , record , self . query . operator , self . query . returning ,) ; statements . push (crate :: debug_query (& stmt) . to_string ()) ; } statements . push ("COMMIT" . into ()) ; f . debug_struct ("Query") . field ("sql" , & statements) . field ("binds" , & [] as & [i32 ; 0]) . finish () } fn fmt_display (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "BEGIN;") ? ; for record in self . query . records . values . iter () { let stmt = InsertStatement :: new (self . query . target , record , self . query . operator , self . query . returning ,) ; writeln ! (f , "{}" , crate :: debug_query (& stmt)) ? ; } writeln ! (f , "COMMIT;") ? ; Ok (()) } }
};
}
