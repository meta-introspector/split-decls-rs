// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [doc = " Convert (&'a Address, &'a mut T) where T: Account into an `AccountInfo`"] impl < 'a , T : Account > IntoAccountInfo < 'a > for (& 'a Address , & 'a mut T) { fn into_account_info (self) -> AccountInfo < 'a > { let (key , account) = self ; let (lamports , data , owner , executable) = account . get () ; AccountInfo :: new (key , false , false , lamports , data , owner , executable) } }
};
}
