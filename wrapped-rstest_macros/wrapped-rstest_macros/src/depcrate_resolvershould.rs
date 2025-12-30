// Generated macro for should (module)
macro_rules! Depcrate_resolvershould {
() => {
// Module: crate::resolver
// Provides: {"should"}
// Dependencies: {}
# [cfg (test)] mod should { use super :: * ; use crate :: test :: { assert_eq , * } ; use syn :: parse_str ; # [test] fn return_the_given_expression () { let ast = parse_str ("fn function(mut foo: String) {}") . unwrap () ; let arg = first_arg_pat (& ast) ; let expected = expr ("bar()") ; let mut resolver = HashMap :: new () ; resolver . insert (pat ("foo") . with_mut () , & expected) ; assert_eq ! (expected , (& resolver) . resolve (& arg) . unwrap () . into_owned ()) } # [test] fn return_the_given_expression_also_if_not_mut_searched () { let ast = parse_str ("fn function(foo: String) {}") . unwrap () ; let arg = first_arg_pat (& ast) ; let expected = expr ("bar()") ; let mut resolver = HashMap :: new () ; resolver . insert (pat ("foo") . with_mut () , & expected) ; assert_eq ! (expected , (& resolver) . resolve (& arg) . unwrap () . into_owned ()) } # [test] fn return_none_for_unknown_argument () { let ast = "fn function(mut fix: String) {}" . ast () ; let arg = first_arg_pat (& ast) ; assert ! (EmptyResolver . resolve (& arg) . is_none ()) } }
};
}
