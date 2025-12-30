// Generated macro for impl_104 (impl)
macro_rules! Depcrate_string_cowimpl_104 {
() => {
// Module: crate::string_cow
// Provides: {"impl_104"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > std :: borrow :: Borrow < str > for KStringCowBase < '_ , B > { # [inline] fn borrow (& self) -> & str { self . as_str () } }
};
}
