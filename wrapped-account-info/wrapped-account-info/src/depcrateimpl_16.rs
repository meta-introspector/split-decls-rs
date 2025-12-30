// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
# [doc = " Convert (&'a Address, bool, &'a mut T)  where T: Account into an"] # [doc = " `AccountInfo`."] impl < 'a , T : Account > IntoAccountInfo < 'a > for (& 'a Address , bool , & 'a mut T) { fn into_account_info (self) -> AccountInfo < 'a > { let (key , is_signer , account) = self ; let (lamports , data , owner , executable) = account . get () ; AccountInfo :: new (key , is_signer , false , lamports , data , owner , executable) } }
};
}
