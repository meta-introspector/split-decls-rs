// Generated macro for impl_9 (impl)
macro_rules! Depcrate_cpiimpl_9 {
() => {
// Module: crate::cpi
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > From < & 'a AccountView > for CpiAccount < 'a > { fn from (account : & 'a AccountView) -> Self { CpiAccount { address : account . address () , lamports : unsafe { & (* account . account_ptr ()) . lamports } , data_len : account . data_len () as u64 , data : account . data_ptr () , owner : unsafe { account . owner () } , rent_epoch : 0 , is_signer : account . is_signer () , is_writable : account . is_writable () , executable : account . executable () , _account_view : PhantomData :: < & 'a AccountView > , } } }
};
}
