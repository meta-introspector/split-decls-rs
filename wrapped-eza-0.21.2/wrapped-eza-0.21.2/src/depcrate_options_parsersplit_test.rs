// Generated macro for split_test (module)
macro_rules! Depcrate_options_parsersplit_test {
() => {
// Module: crate::options::parser
// Provides: {"split_test"}
// Dependencies: {}
# [cfg (test)] mod split_test { use super :: split_on_equals ; use std :: ffi :: { OsStr , OsString } ; macro_rules ! test_split { ($ name : ident : $ input : expr => None) => { # [test] fn $ name () { assert_eq ! (split_on_equals (& OsString :: from ($ input)) , None) ; } } ; ($ name : ident : $ input : expr => $ before : expr , $ after : expr) => { # [test] fn $ name () { assert_eq ! (split_on_equals (& OsString :: from ($ input)) , Some ((OsStr :: new ($ before) , OsStr :: new ($ after)))) ; } } ; } test_split ! (empty : "" => None) ; test_split ! (letter : "a" => None) ; test_split ! (just : "=" => None) ; test_split ! (intro : "=bbb" => None) ; test_split ! (denou : "aaa=" => None) ; test_split ! (equals : "aaa=bbb" => "aaa" , "bbb") ; test_split ! (sort : "--sort=size" => "--sort" , "size") ; test_split ! (more : "this=that=other" => "this" , "that=other") ; }
};
}
