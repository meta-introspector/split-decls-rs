// Generated macro for test_trait_retainable (function)
macro_rules! Depcrate_tests_arraytest_trait_retainable {
() => {
// Module: crate::tests::array
// Provides: {"test_trait_retainable"}
// Dependencies: {}
# [test] fn test_trait_retainable () { extern_protocol ! (# [allow (clippy :: missing_safety_doc)] # [name = "NSObject"] unsafe trait TestProtocol { }) ; unsafe impl TestProtocol for NSNumber { } let obj : Retained < ProtocolObject < dyn TestProtocol > > = ProtocolObject :: from_retained (NSNumber :: new_i32 (42)) ; let _ = NSArray :: from_slice (& [& * obj , & * obj]) ; let _ = NSArray :: from_retained_slice (& [obj . clone () , obj . clone ()]) ; }
};
}
