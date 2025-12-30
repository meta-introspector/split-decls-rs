// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_query_builder_sql_queryimpl_1319 {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"impl_1319"}
// Dependencies: {}
impl < 'f , DB : Backend , Query > BoxedSqlQuery < 'f , DB , Query > { pub (crate) fn new (query : Query) -> Self { BoxedSqlQuery { query , sql : "" . to_string () , binds : vec ! [] , } } # [doc = " See [`SqlQuery::bind`]."] # [doc = ""] # [doc = " [`SqlQuery::bind`]: SqlQuery::bind()"] pub fn bind < BindSt , Value > (mut self , b : Value) -> Self where DB : HasSqlType < BindSt > , Value : ToSql < BindSt , DB > + Send + 'f , BindSt : Send + 'f , { self . binds . push (Box :: new (RawBind { value : b , p : PhantomData , }) as Box < _ >) ; self } # [doc = " See [`SqlQuery::sql`]."] # [doc = ""] # [doc = " [`SqlQuery::sql`]: SqlQuery::sql()"] pub fn sql < T : AsRef < str > > (mut self , sql : T) -> Self { self . sql += sql . as_ref () ; self } }
};
}
