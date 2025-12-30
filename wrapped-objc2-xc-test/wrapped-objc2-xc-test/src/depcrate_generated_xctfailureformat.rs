// Generated macro for _XCTFailureFormat (function)
macro_rules! Depcrate_generated_XCTFailureFormat {
() => {
// Module: crate::generated
// Provides: {"_XCTFailureFormat"}
// Dependencies: {}
# [inline] pub (crate) extern "C-unwind" fn _XCTFailureFormat (assertion_type : _XCTAssertionType , format_index : NSUInteger ,) -> Retained < NSString > { extern "C-unwind" { fn _XCTFailureFormat (assertion_type : _XCTAssertionType , format_index : NSUInteger ,) -> * mut NSString ; } let ret = unsafe { _XCTFailureFormat (assertion_type , format_index) } ; unsafe { Retained :: retain_autoreleased (ret) } . expect ("function was marked as returning non-null, but actually returned NULL") }
};
}
