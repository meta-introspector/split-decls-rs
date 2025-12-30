// Generated macro for impl_30 (impl)
macro_rules! Depcrate_address_channelimpl_30 {
() => {
// Module: crate::address::channel
// Provides: {"impl_30"}
// Dependencies: {}
impl < A : Actor > fmt :: Debug for AddressSender < A > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("AddressSender") . field ("sender_task" , & self . sender_task) . field ("maybe_parked" , & self . maybe_parked) . finish () } }
};
}
