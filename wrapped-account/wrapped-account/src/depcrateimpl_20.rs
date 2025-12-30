// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl From < AccountSharedData > for Account { fn from (mut other : AccountSharedData) -> Self { let account_data = Arc :: make_mut (& mut other . data) ; Self { lamports : other . lamports , data : std :: mem :: take (account_data) , owner : other . owner , executable : other . executable , rent_epoch : other . rent_epoch , } } }
};
}
