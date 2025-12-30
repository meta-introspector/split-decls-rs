// Generated macro for tests (module)
macro_rules! Depcrate_parsetests {
() => {
// Module: crate::parse
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_number_lookahead_restrictions () { GraphQLParser :: parse (Rule :: const_list , "[123 abc]") . unwrap () ; GraphQLParser :: parse (Rule :: const_list , "[123.0123 abc]") . unwrap () ; GraphQLParser :: parse (Rule :: const_list , "[123.0123e7 abc]") . unwrap () ; GraphQLParser :: parse (Rule :: const_list , "[123.0123e77 abc]") . unwrap () ; assert ! (GraphQLParser :: parse (Rule :: const_list , "[123abc]") . is_err ()) ; assert ! (GraphQLParser :: parse (Rule :: const_list , "[123.0123abc]") . is_err ()) ; assert ! (GraphQLParser :: parse (Rule :: const_list , "[123.0123e7abc]") . is_err ()) ; assert ! (GraphQLParser :: parse (Rule :: const_list , "[123.0123e77abc]") . is_err ()) ; } }
};
}
