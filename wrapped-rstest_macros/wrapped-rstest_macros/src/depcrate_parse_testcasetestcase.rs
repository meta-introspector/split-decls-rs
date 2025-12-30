// Generated macro for TestCase (struct)
macro_rules! Depcrate_parse_testcaseTestCase {
() => {
// Module: crate::parse::testcase
// Provides: {"TestCase"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone)] # [doc = " A test case instance data. Contains a list of arguments. It is parsed by parametrize"] # [doc = " attributes."] pub (crate) struct TestCase { pub (crate) args : Vec < Expr > , pub (crate) attrs : Vec < Attribute > , pub (crate) description : Option < Ident > , }
};
}
