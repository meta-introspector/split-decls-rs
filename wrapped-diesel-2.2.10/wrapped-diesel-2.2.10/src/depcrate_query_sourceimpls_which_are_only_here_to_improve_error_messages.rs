// Generated macro for impls_which_are_only_here_to_improve_error_messages (module)
macro_rules! Depcrate_query_sourceimpls_which_are_only_here_to_improve_error_messages {
() => {
// Module: crate::query_source
// Provides: {"impls_which_are_only_here_to_improve_error_messages"}
// Dependencies: {}
# [doc (hidden)] # [allow (non_camel_case_types , missing_debug_implementations , missing_copy_implementations)] # [doc = " Everything in this module is here to give something more helpful than:"] # [doc = ""] # [doc = " > (Never, Never): Pick<table1, table2> is not satisfied"] # [doc = ""] # [doc = " Any of these impls can be deleted if they are getting in the way of"] # [doc = " other functionality. Any code which is using these impls is already"] # [doc = " failing to compile."] mod impls_which_are_only_here_to_improve_error_messages { use super :: * ; pub struct this_table_doesnt_appear_in_the_from_clause_of_your_query ; impl < Left , Right > Pick < Left , Right > for (Never , Never) { type Selection = this_table_doesnt_appear_in_the_from_clause_of_your_query ; } pub struct this_table_appears_in_your_query_more_than_once_and_must_be_aliased ; impl < Left , Right , OtherCount > Pick < Left , Right > for (MoreThanOnce , OtherCount) { type Selection = this_table_appears_in_your_query_more_than_once_and_must_be_aliased ; } impl < Left , Right > Pick < Left , Right > for (Never , MoreThanOnce) { type Selection = this_table_appears_in_your_query_more_than_once_and_must_be_aliased ; } impl < Left , Right > Pick < Left , Right > for (Once , MoreThanOnce) { type Selection = this_table_appears_in_your_query_more_than_once_and_must_be_aliased ; } }
};
}
