// Generated macro for impl_369 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_369 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_369"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: PoolDisabled => "pool is disabled" , Error :: CheckedOutClosedValue => "checked out connection was closed" , Error :: CheckoutNoLongerWanted => "request was canceled" , }) } }
};
}
