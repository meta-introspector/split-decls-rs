// Generated macro for BoxedSelectStatement (struct)
macro_rules! Depcrate_query_builder_select_statement_boxedBoxedSelectStatement {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"BoxedSelectStatement"}
// Dependencies: {}
# [doc = " This type represents a boxed select query"] # [doc = ""] # [doc = " Using this type directly is only meaningful for custom backends"] # [doc = " that need to provide a custom [`QueryFragment`] implementation"] # [allow (missing_debug_implementations)] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , public_fields (select , from , distinct , where_clause , order , limit_offset , group_by , having))] pub struct BoxedSelectStatement < 'a , ST , QS , DB , GB = () > { # [doc = " The select clause of the query"] select : Box < dyn QueryFragment < DB > + Send + 'a > , # [doc = " The from clause of the query"] from : QS , # [doc = " The distinct clause of the query"] distinct : Box < dyn QueryFragment < DB > + Send + 'a > , # [doc = " The where clause of the query"] where_clause : BoxedWhereClause < 'a , DB > , # [doc = " The order clause of the query"] order : Option < Box < dyn QueryFragment < DB > + Send + 'a > > , # [doc = " The combined limit/offset clause of the query"] limit_offset : BoxedLimitOffsetClause < 'a , DB > , # [doc = " The group by clause of the query"] group_by : Box < dyn QueryFragment < DB > + Send + 'a > , # [doc = " The having clause of the query"] having : Box < dyn QueryFragment < DB > + Send + 'a > , _marker : PhantomData < (ST , GB) > , }
};
}
