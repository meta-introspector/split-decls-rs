// Generated macro for impl_1732 (impl)
macro_rules! Depcrate_tlsimpl_1732 {
() => {
// Module: crate::tls
// Provides: {"impl_1732"}
// Dependencies: {}
impl < 'a > ExData < 'a > { fn from_ssl_ptr (ptr : * const SSL) -> Option < & 'a mut Self > { get_ex_data_from_ptr :: < ExData > (ptr , * QUICHE_EX_DATA_INDEX) } # [cfg (feature = "boringssl-boring-crate")] pub fn from_ssl_ref (ssl : & mut boring :: ssl :: SslRef) -> Option < & mut Self > { use boring :: ex_data :: Index ; let idx : Index < boring :: ssl :: Ssl , ExData > = unsafe { Index :: from_raw (* QUICHE_EX_DATA_INDEX) } ; ssl . ex_data_mut (idx) } }
};
}
