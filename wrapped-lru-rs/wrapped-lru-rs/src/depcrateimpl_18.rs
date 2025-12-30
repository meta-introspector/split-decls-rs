// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < K : PartialEq > PartialEq for KeyRef < K > { # ! [allow (unknown_lints)] # [allow (clippy :: unconditional_recursion)] fn eq (& self , other : & KeyRef < K >) -> bool { unsafe { (* self . k) . eq (& * other . k) } } }
};
}
