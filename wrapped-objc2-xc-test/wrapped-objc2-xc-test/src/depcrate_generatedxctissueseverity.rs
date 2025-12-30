// Generated macro for XCTIssueSeverity (struct)
macro_rules! Depcrate_generatedXCTIssueSeverity {
() => {
// Module: crate::generated
// Provides: {"XCTIssueSeverity"}
// Dependencies: {}
# [doc = " An enum representing the severity of a test issue."] # [doc = ""] # [doc = " The numeric values of this enum's cases are comparable. A case which represents"] # [doc = " higher severity has a larger numeric value than one which represents lower"] # [doc = " severity. Specifying a numeric severity value other than one corresponding to"] # [doc = " a case defined below when initializing an ``XCTIssue`` is unsupported."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/xctest/xctissueseverity?language=objc)"] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct XCTIssueSeverity (pub NSInteger) ;
};
}
