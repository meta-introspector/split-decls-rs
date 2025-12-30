// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < T > KeyIvInit for T where T : InnerIvInit , T :: Inner : KeyInit , { # [inline] fn new (key : & Key < Self > , iv : & Iv < Self >) -> Self { Self :: inner_iv_init (T :: Inner :: new (key) , iv) } # [inline] fn new_from_slices (key : & [u8] , iv : & [u8]) -> Result < Self , InvalidLength > { T :: Inner :: new_from_slice (key) . and_then (| i | T :: inner_iv_slice_init (i , iv)) } # [inline] fn weak_key_test (key : & Key < Self >) -> Result < () , WeakKeyError > { T :: Inner :: weak_key_test (key) } }
};
}
