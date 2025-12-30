// Generated macro for test_bitfields_third (function)
macro_rules! Depcratetest_bitfields_third {
() => {
// Module: crate
// Provides: {"test_bitfields_third"}
// Dependencies: {}
# [test] fn test_bitfields_third () { let mut third : bindings :: bitfields :: Third = unsafe { mem :: zeroed () } ; assert ! (unsafe { third . assert (0 , false , bindings :: bitfields :: ItemKind :: ITEM_KIND_UNO) }) ; third . set_flags (12345) ; third . set_is_whatever (true) ; third . set_kind (bindings :: bitfields :: ItemKind :: ITEM_KIND_TRES) ; assert ! (unsafe { third . assert (12345 , true , bindings :: bitfields :: ItemKind :: ITEM_KIND_TRES) }) ; }
};
}
