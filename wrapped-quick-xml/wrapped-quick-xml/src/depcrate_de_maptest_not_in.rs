// Generated macro for test_not_in (function)
macro_rules! Depcrate_de_maptest_not_in {
() => {
// Module: crate::de::map
// Provides: {"test_not_in"}
// Dependencies: {}
# [test] fn test_not_in () { use pretty_assertions :: assert_eq ; let tag = BytesStart :: new ("tag") ; assert_eq ! (not_in (& [] , & tag) . unwrap () , true) ; assert_eq ! (not_in (& ["no" , "such" , "tags"] , & tag) . unwrap () , true) ; assert_eq ! (not_in (& ["some" , "tag" , "included"] , & tag) . unwrap () , false) ; let tag_ns = BytesStart :: new ("ns1:tag") ; assert_eq ! (not_in (& ["no" , "such" , "tags"] , & tag_ns) . unwrap () , true) ; assert_eq ! (not_in (& ["some" , "tag" , "included"] , & tag_ns) . unwrap () , false) ; assert_eq ! (not_in (& ["some" , "namespace" , "ns1:tag"] , & tag_ns) . unwrap () , true) ; }
};
}
