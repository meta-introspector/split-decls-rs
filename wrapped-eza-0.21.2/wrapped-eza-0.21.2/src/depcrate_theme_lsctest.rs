// Generated macro for test (module)
macro_rules! Depcrate_theme_lsctest {
() => {
// Module: crate::theme::lsc
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; macro_rules ! test { ($ name : ident : $ input : expr => $ result : expr) => { # [test] fn $ name () { let mut lscs = Vec :: new () ; LSColors ($ input) . each_pair (| p | lscs . push ((p . key . clone () , p . to_style ()))) ; assert_eq ! (lscs , $ result . to_vec ()) ; } } ; } test ! (empty : "" => []) ; test ! (jibber : "blah" => []) ; test ! (equals : "=" => []) ; test ! (starts : "=di" => []) ; test ! (ends : "id=" => []) ; test ! (green : "cb=32" => [("cb" , Green . normal ())]) ; test ! (red : "di=31" => [("di" , Red . normal ())]) ; test ! (blue : "la=34" => [("la" , Blue . normal ())]) ; test ! (yellow : "do=43" => [("do" , Style :: default () . on (Yellow))]) ; test ! (purple : "re=45" => [("re" , Style :: default () . on (Purple))]) ; test ! (cyan : "mi=46" => [("mi" , Style :: default () . on (Cyan))]) ; test ! (bold : "fa=1" => [("fa" , Style :: default () . bold ())]) ; test ! (under : "so=4" => [("so" , Style :: default () . underline ())]) ; test ! (both : "la=1;4" => [("la" , Style :: default () . bold () . underline ())]) ; test ! (more : "me=43;21;55;34:yu=1;4;1" => [("me" , Blue . on (Yellow)) , ("yu" , Style :: default () . bold () . underline ())]) ; test ! (many : "red=31:green=32:blue=34" => [("red" , Red . normal ()) , ("green" , Green . normal ()) , ("blue" , Blue . normal ())]) ; }
};
}
