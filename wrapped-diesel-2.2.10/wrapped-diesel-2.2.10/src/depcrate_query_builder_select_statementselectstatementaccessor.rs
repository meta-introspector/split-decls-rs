// Generated macro for SelectStatementAccessor (trait)
macro_rules! Depcrate_query_builder_select_statementSelectStatementAccessor {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"SelectStatementAccessor"}
// Dependencies: {}
# [doc = " Semi-Private trait for containing get-functions for all `SelectStatement` fields"] pub trait SelectStatementAccessor { # [doc = " The type of the select clause"] type Select ; # [doc = " The type of the from clause"] type From ; # [doc = " The type of the distinct clause"] type Distinct ; # [doc = " The type of the where clause"] type Where ; # [doc = " The type of the order clause"] type Order ; # [doc = " The type of the limit offset clause"] type LimitOffset ; # [doc = " The type of the group by clause"] type GroupBy ; # [doc = " The type of the having clause"] type Having ; # [doc = " The type of the locking clause"] type Locking ; # [doc = " Access the select clause"] fn select_clause (& self) -> & Self :: Select ; # [doc = " Access the from clause"] # [allow (clippy :: wrong_self_convention)] fn from_clause (& self) -> & Self :: From ; # [doc = " Access the distinct clause"] fn distinct_clause (& self) -> & Self :: Distinct ; # [doc = " Access the where clause"] fn where_clause (& self) -> & Self :: Where ; # [doc = " Access the order clause"] fn order_clause (& self) -> & Self :: Order ; # [doc = " Access the limit_offset clause"] fn limit_offset_clause (& self) -> & Self :: LimitOffset ; # [doc = " Access the group by clause"] fn group_by_clause (& self) -> & Self :: GroupBy ; # [doc = " Access the having clause"] fn having_clause (& self) -> & Self :: Having ; # [doc = " Access the locking clause"] fn locking_clause (& self) -> & Self :: Locking ; }
};
}
