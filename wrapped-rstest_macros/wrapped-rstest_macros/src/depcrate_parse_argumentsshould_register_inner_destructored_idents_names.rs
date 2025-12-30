// Generated macro for should_register_inner_destructored_idents_names (module)
macro_rules! Depcrate_parse_argumentsshould_register_inner_destructored_idents_names {
() => {
// Module: crate::parse::arguments
// Provides: {"should_register_inner_destructored_idents_names"}
// Dependencies: {}
# [cfg (test)] mod should_register_inner_destructored_idents_names { use super :: * ; use crate :: test :: { assert_eq , * } ; # [test] fn implement_the_correct_pat_reolver () { let item_fn = "fn test_function(A(a,b): A, (c,d,e): (u32, u32, u32), none: u32, B{s,d} : B, clean: C) {}" . ast () ; let mut arguments = ArgumentsInfo :: default () ; arguments . register_inner_destructored_idents_names (& item_fn) ; assert_eq ! (arguments . inner_pat (& pat ("A(a,b)")) , & pat ("__destruct_1")) ; assert_eq ! (arguments . inner_pat (& pat ("(c,d,e)")) , & pat ("__destruct_2")) ; assert_eq ! (arguments . inner_pat (& pat ("none")) , & pat ("none")) ; assert_eq ! (arguments . inner_pat (& pat ("B{s,d}")) , & pat ("__destruct_3")) ; assert_eq ! (arguments . inner_pat (& pat ("clean")) , & pat ("clean")) ; } # [test] fn and_replace_them_correctly () { let item_fn = "fn test_function(A(a,b): A, (c,d,e): (u32, u32, u32), none: u32, B{s,d} : B, clean: C) {}" . ast () ; let mut arguments = ArgumentsInfo :: default () ; arguments . register_inner_destructored_idents_names (& item_fn) ; let new_args = arguments . replace_fn_args_with_related_inner_pat (item_fn . sig . inputs . into_iter ()) . filter_map (| f | f . maybe_ident () . cloned ()) . map (| id | id . to_string ()) . collect :: < Vec < _ > > () . join (" | ") ; assert_eq ! (new_args , "__destruct_1 | __destruct_2 | none | __destruct_3 | clean") ; } }
};
}
