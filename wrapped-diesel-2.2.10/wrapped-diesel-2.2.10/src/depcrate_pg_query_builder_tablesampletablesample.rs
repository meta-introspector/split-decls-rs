// Generated macro for Tablesample (struct)
macro_rules! Depcrate_pg_query_builder_tablesampleTablesample {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"Tablesample"}
// Dependencies: {}
# [doc = " Represents a query with a `TABLESAMPLE` clause."] # [derive (Debug , Clone , Copy)] pub struct Tablesample < S , TSM > where TSM : TablesampleMethod , { source : S , method : PhantomData < TSM > , portion : i16 , seed : Option < f64 > , }
};
}
