// Generated macro for alt_dynamic_array (function)
macro_rules! Depcrate_branch_testsalt_dynamic_array {
() => {
// Module: crate::branch::tests
// Provides: {"alt_dynamic_array"}
// Dependencies: {}
# [test] fn alt_dynamic_array () { fn alt1 (i : & [u8]) -> IResult < & [u8] , & [u8] > { alt (& mut [tag ("a") , tag ("bc") , tag ("def")] [..]) . parse (i) } let a = & b"a" [..] ; assert_eq ! (alt1 (a) , Ok ((& b"" [..] , (& b"a" [..])))) ; let bc = & b"bc" [..] ; assert_eq ! (alt1 (bc) , Ok ((& b"" [..] , (& b"bc" [..])))) ; let defg = & b"defg" [..] ; assert_eq ! (alt1 (defg) , Ok ((& b"g" [..] , (& b"def" [..])))) ; }
};
}
