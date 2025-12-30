// Generated macro for cbb_to_buffer (function)
macro_rules! Depcratecbb_to_buffer {
() => {
// Module: crate
// Provides: {"cbb_to_buffer"}
// Dependencies: {}
# [doc = " Calls `func` with a `CBB` pointer and returns a [Buffer] of the ultimate"] # [doc = " contents of that CBB."] # [allow (clippy :: unwrap_used)] fn cbb_to_buffer < F : FnOnce (* mut bssl_sys :: CBB) > (initial_capacity : usize , func : F) -> Buffer { let mut cbb = unsafe { initialized_struct_fallible (| cbb | bssl_sys :: CBB_init (cbb , initial_capacity) == 1) } . unwrap () ; func (& mut cbb) ; let mut ptr : * mut u8 = core :: ptr :: null_mut () ; let mut len : usize = 0 ; assert_eq ! (1 , unsafe { bssl_sys :: CBB_finish (& mut cbb , & mut ptr , & mut len) }) ; unsafe { Buffer :: new (ptr , len) } }
};
}
