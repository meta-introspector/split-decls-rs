// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: helpers :: registry ; # [test] fn meta_test_deep_pretty_print_registry () { assert_eq ! (& format ! ("{:?}" , PrettyPrintRegistry (vec ! [pkg ! (("foo" , "1.0.1") => [dep_req ("bar" , "1")]) , pkg ! (("foo" , "1.0.0") => [dep_req ("bar" , "2")]) , pkg ! (("foo" , "2.0.0") => [dep_req ("bar" , "*")]) , pkg ! (("bar" , "1.0.0") => [dep_req ("baz" , "=1.0.2") , dep_req ("other" , "1")]) , pkg ! (("bar" , "2.0.0") => [dep_req ("baz" , "=1.0.1")]) , pkg ! (("baz" , "1.0.2") => [dep_req ("other" , "2")]) , pkg ! (("baz" , "1.0.1")) , pkg ! (("cat" , "1.0.2") => [dep_req_kind ("other" , "2" , DepKind :: Build)]) , pkg ! (("cat" , "1.0.3") => [dep_req_kind ("other" , "2" , DepKind :: Development)]) , pkg ! (("dep_req" , "1.0.0")) , pkg ! (("dep_req" , "2.0.0")) ,])) , "vec![pkg!((\"foo\", \"1.0.1\") => [dep_req(\"bar\", \"^1\"),]),\
         pkg!((\"foo\", \"1.0.0\") => [dep_req(\"bar\", \"^2\"),]),\
         pkg!((\"foo\", \"2.0.0\") => [dep(\"bar\"),]),\
         pkg!((\"bar\", \"1.0.0\") => [dep_req(\"baz\", \"=1.0.2\"),dep_req(\"other\", \"^1\"),]),\
         pkg!((\"bar\", \"2.0.0\") => [dep_req(\"baz\", \"=1.0.1\"),]),\
         pkg!((\"baz\", \"1.0.2\") => [dep_req(\"other\", \"^2\"),]),\
         pkg!((\"baz\", \"1.0.1\")),\
         pkg!((\"cat\", \"1.0.2\") => [dep_req_kind(\"other\", \"^2\", DepKind::Build, false),]),\
         pkg!((\"cat\", \"1.0.3\") => [dep_req_kind(\"other\", \"^2\", DepKind::Development, false),]),\
         pkg!((\"dep_req\", \"1.0.0\")),\
         pkg!((\"dep_req\", \"2.0.0\")),]") } # [doc = " This test is to test the generator to ensure"] # [doc = " that it makes registries with large dependency trees"] # [test] fn meta_test_deep_trees_from_strategy () { use proptest :: strategy :: ValueTree ; use proptest :: test_runner :: TestRunner ; let mut dis = [0 ; 21] ; let strategy = registry_strategy (50 , 20 , 60) ; let mut test_runner = TestRunner :: deterministic () ; for _ in 0 .. 128 { let PrettyPrintRegistry (input) = strategy . new_tree (& mut TestRunner :: new_with_rng (Default :: default () , test_runner . new_rng () ,)) . unwrap () . current () ; let reg = registry (input . clone ()) ; for this in input . iter () . rev () . take (10) { let res = resolve (vec ! [dep_req (& this . name () , & format ! ("={}" , this . version ()))] , & reg ,) ; dis [res . as_ref () . map (| x | min (x . len () , dis . len ()) - 1) . unwrap_or (0)] += 1 ; if dis . iter () . all (| & x | x > 0) { return ; } } } panic ! ("In 1280 tries we did not see a wide enough distribution \
             of dependency trees! dis: {dis:?}") ; } # [doc = " This test is to test the generator to ensure"] # [doc = " that it makes registries that include multiple versions of the same library"] # [test] fn meta_test_multiple_versions_strategy () { use proptest :: strategy :: ValueTree ; use proptest :: test_runner :: TestRunner ; let mut dis = [0 ; 10] ; let strategy = registry_strategy (50 , 20 , 60) ; let mut test_runner = TestRunner :: deterministic () ; for _ in 0 .. 128 { let PrettyPrintRegistry (input) = strategy . new_tree (& mut TestRunner :: new_with_rng (Default :: default () , test_runner . new_rng () ,)) . unwrap () . current () ; let reg = registry (input . clone ()) ; for this in input . iter () . rev () . take (10) { let res = resolve (vec ! [dep_req (& this . name () , & format ! ("={}" , this . version ()))] , & reg ,) ; if let Ok (mut res) = res { let res_len = res . len () ; res . sort_by_key (| s | s . name ()) ; res . dedup_by_key (| s | s . name ()) ; dis [min (res_len - res . len () , dis . len () - 1)] += 1 ; } if dis . iter () . all (| & x | x > 0) { return ; } } } panic ! ("In 1280 tries we did not see a wide enough distribution \
             of multiple versions of the same library! dis: {dis:?}") ; } }
};
}
