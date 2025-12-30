// Generated macro for JoinOnDsl (trait)
macro_rules! Depcrate_query_dsl_join_dslJoinOnDsl {
() => {
// Module: crate::query_dsl::join_dsl
// Provides: {"JoinOnDsl"}
// Dependencies: {}
# [doc = " Specify the `ON` clause for a join statement. This will override"] # [doc = " any implicit `ON` clause that would come from [`joinable!`]"] # [doc = ""] # [doc = " [`joinable!`]: crate::joinable!"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " # use schema::{users, posts};"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let data = users::table"] # [doc = "     .left_join(posts::table.on("] # [doc = "         users::id.eq(posts::user_id).and("] # [doc = "             posts::title.eq(\"My first post\"))"] # [doc = "     ))"] # [doc = "     .select((users::name, posts::title.nullable()))"] # [doc = "     .load(connection);"] # [doc = " let expected = vec!["] # [doc = "     (\"Sean\".to_string(), Some(\"My first post\".to_string())),"] # [doc = "     (\"Tess\".to_string(), None),"] # [doc = " ];"] # [doc = " assert_eq!(Ok(expected), data);"] # [doc = " # }"] pub trait JoinOnDsl : Sized { # [doc = " See the trait documentation."] fn on < On > (self , on : On) -> helper_types :: On < Self , On > { OnClauseWrapper :: new (self , on) } }
};
}
