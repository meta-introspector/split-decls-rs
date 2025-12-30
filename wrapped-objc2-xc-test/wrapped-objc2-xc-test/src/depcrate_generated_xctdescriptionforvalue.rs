// Generated macro for _XCTDescriptionForValue (function)
macro_rules! Depcrate_generated_XCTDescriptionForValue {
() => {
// Module: crate::generated
// Provides: {"_XCTDescriptionForValue"}
// Dependencies: {}
# [inline] pub (crate) extern "C-unwind" fn _XCTDescriptionForValue (value : & NSValue) -> Retained < NSString > { extern "C-unwind" { fn _XCTDescriptionForValue (value : & NSValue) -> * mut NSString ; } let ret = unsafe { _XCTDescriptionForValue (value) } ; unsafe { Retained :: retain_autoreleased (ret) } . expect ("function was marked as returning non-null, but actually returned NULL") }
};
}
