// Generated macro for test_compile (function)
macro_rules! Depcrate_argtest_compile {
() => {
// Module: crate::arg
// Provides: {"test_compile"}
// Dependencies: {}
# [allow (dead_code)] fn test_compile () { let mut msg = Message :: new_signal ("/" , "a.b" , "C") . unwrap () ; let mut q = IterAppend :: new (& mut msg) ; q . append (5u8) ; q . append (Array :: new (& [5u8 , 6 , 7])) ; q . append ((8u8 , & [9u8 , 6 , 7] [..])) ; q . append (Variant ((6u8 , 7u8))) ; }
};
}
