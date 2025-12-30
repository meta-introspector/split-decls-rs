// Generated macro for DuplicatedKeys (struct)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsDuplicatedKeys {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"DuplicatedKeys"}
// Dependencies: {}
# [doc = " A marker type signaling that the given `ON CONFLICT` clause"] # [doc = " uses mysql's `ON DUPLICATE KEY` syntax that triggers on"] # [doc = " all unique constraints"] # [doc = ""] # [doc = " See [`InsertStatement::on_conflict`](crate::query_builder::InsertStatement::on_conflict)"] # [doc = " for examples"] # [derive (Debug , Copy , Clone)] pub struct DuplicatedKeys ;
};
}
