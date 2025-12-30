// Generated macro for Expect (struct)
macro_rules! Depcrate_common_expectExpect {
() => {
// Module: crate::common::expect
// Provides: {"Expect"}
// Dependencies: {}
# [doc = " The `Expect` header."] # [doc = ""] # [doc = " > The \"Expect\" header field in a request indicates a certain set of"] # [doc = " > behaviors (expectations) that need to be supported by the server in"] # [doc = " > order to properly handle this request.  The only such expectation"] # [doc = " > defined by this specification is 100-continue."] # [doc = " >"] # [doc = " >    Expect  = \"100-continue\""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Expect;"] # [doc = ""] # [doc = " let expect = Expect::CONTINUE;"] # [doc = " ```"] # [derive (Clone , PartialEq)] pub struct Expect (()) ;
};
}
