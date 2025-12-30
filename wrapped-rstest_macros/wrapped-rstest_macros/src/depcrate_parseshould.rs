// Generated macro for should (module)
macro_rules! Depcrate_parseshould {
() => {
// Module: crate::parse
// Provides: {"should"}
// Dependencies: {}
# [cfg (test)] mod should { use super :: * ; use crate :: test :: * ; mod parse_attributes { use super :: assert_eq ; use super :: * ; fn parse_attributes < S : AsRef < str > > (attributes : S) -> Attributes { parse_meta (attributes) } # [test] fn one_simple_ident () { let attributes = parse_attributes ("my_ident") ; let expected = Attributes { attributes : vec ! [Attribute :: attr ("my_ident")] , } ; assert_eq ! (expected , attributes) ; } # [test] fn one_simple_group () { let attributes = parse_attributes ("group_tag(first, second)") ; let expected = Attributes { attributes : vec ! [Attribute :: tagged ("group_tag" , vec ! ["first" , "second"])] , } ; assert_eq ! (expected , attributes) ; } # [test] fn one_simple_type () { let attributes = parse_attributes ("type_tag<(u32, T, (String, i32))>") ; let expected = Attributes { attributes : vec ! [Attribute :: typed ("type_tag" , "(u32, T, (String, i32))")] , } ; assert_eq ! (expected , attributes) ; } # [test] fn integrated () { let attributes = parse_attributes (r#"
            simple :: tagged(first, second) :: type_tag<(u32, T, (std::string::String, i32))> :: more_tagged(a,b)"# ,) ; let expected = Attributes { attributes : vec ! [Attribute :: attr ("simple") , Attribute :: tagged ("tagged" , vec ! ["first" , "second"]) , Attribute :: typed ("type_tag" , "(u32, T, (std::string::String, i32))") , Attribute :: tagged ("more_tagged" , vec ! ["a" , "b"]) ,] , } ; assert_eq ! (expected , attributes) ; } } }
};
}
