// Generated macro for impl_1023 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1023 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1023"}
// Dependencies: {}
impl < T : QuerySource , U , C , Op , Ret > InsertStatement < T , InsertFromSelect < U , C > , Op , Ret > { # [doc = " Set the column list when inserting from a select statement"] # [doc = ""] # [doc = " See the documentation for [`insert_into`] for usage examples."] # [doc = ""] # [doc = " [`insert_into`]: crate::insert_into()"] pub fn into_columns < C2 > (self , columns : C2 ,) -> InsertStatement < T , InsertFromSelect < U , C2 > , Op , Ret > where C2 : ColumnList < Table = T > + Expression , U : Query < SqlType = C2 :: SqlType > , { InsertStatement :: new (self . target , self . records . with_columns (columns) , self . operator , self . returning ,) } }
};
}
