// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl From < Account > for AccountSharedData { fn from (other : Account) -> Self { Self { lamports : other . lamports , data : Arc :: new (other . data) , owner : other . owner , executable : other . executable , rent_epoch : other . rent_epoch , } } }
};
}
