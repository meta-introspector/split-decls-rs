// Generated macro for impl_258 (impl)
macro_rules! Depcrate_drivers_net_loopbackimpl_258 {
() => {
// Module: crate::drivers::net::loopback
// Provides: {"impl_258"}
// Dependencies: {}
impl smoltcp :: phy :: Device for LoopbackDriver { type RxToken < 'a > = RxToken < 'a > ; type TxToken < 'a > = TxToken < 'a > ; fn receive (& mut self , _ : Instant) -> Option < (RxToken < '_ > , TxToken < '_ >) > { if self . queue . lock () . len () > self . reserved_receives . load (Ordering :: Relaxed) { self . reserved_receives . fetch_add (1 , Ordering :: Relaxed) ; Some ((RxToken { queue : & self . queue , reserved_receives : & self . reserved_receives , } , TxToken { queue : & self . queue } ,)) } else { None } } fn transmit (& mut self , _ : Instant) -> Option < TxToken < '_ > > { Some (TxToken { queue : & self . queue }) } fn capabilities (& self) -> phy :: DeviceCapabilities { let mut capabilities = phy :: DeviceCapabilities :: default () ; capabilities . medium = phy :: Medium :: Ethernet ; capabilities . max_transmission_unit = u16 :: MAX . into () ; capabilities } }
};
}
