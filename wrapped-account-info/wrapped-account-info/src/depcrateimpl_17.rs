// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [doc = " Convert &'a mut (Address, T) where T: Account into an `AccountInfo`."] impl < 'a , T : Account > IntoAccountInfo < 'a > for & 'a mut (Address , T) { fn into_account_info (self) -> AccountInfo < 'a > { let (ref key , account) = self ; let (lamports , data , owner , executable) = account . get () ; AccountInfo :: new (key , false , false , lamports , data , owner , executable) } }
};
}
