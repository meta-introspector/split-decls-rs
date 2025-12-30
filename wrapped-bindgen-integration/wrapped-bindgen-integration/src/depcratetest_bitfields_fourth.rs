// Generated macro for test_bitfields_fourth (function)
macro_rules! Depcratetest_bitfields_fourth {
() => {
// Module: crate
// Provides: {"test_bitfields_fourth"}
// Dependencies: {}
# [test] fn test_bitfields_fourth () { let mut fourth : bindings :: bitfields :: Fourth = unsafe { mem :: zeroed () } ; assert ! (unsafe { fourth . assert (bindings :: bitfields :: MyEnum :: ONE , 0) }) ; fourth . set_tag (bindings :: bitfields :: MyEnum :: THREE) ; fourth . set_ptr (0xdeadbeef) ; assert ! (unsafe { fourth . assert (bindings :: bitfields :: MyEnum :: THREE , 0xdeadbeef) }) ; }
};
}
