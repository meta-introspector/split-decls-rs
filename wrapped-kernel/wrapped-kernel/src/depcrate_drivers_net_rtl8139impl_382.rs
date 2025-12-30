// Generated macro for impl_382 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_382 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_382"}
// Dependencies: {}
impl smoltcp :: phy :: Device for RTL8139Driver { type RxToken < 'a > = RxToken < 'a > ; type TxToken < 'a > = TxToken < 'a > ; fn receive (& mut self , _ : smoltcp :: time :: Instant) -> Option < (RxToken < '_ > , TxToken < '_ >) > { if ! self . rx_fields . rx_in_use && self . has_packet () { self . rx_fields . rx_in_use = true ; let regs = self . regs . as_mut_ptr () ; Some ((RxToken { capr : map_field ! (regs . capr) , rx_fields : & mut self . rx_fields , } , TxToken { tsd0 : map_field ! (regs . tsd0) , tsd1 : map_field ! (regs . tsd1) , tsd2 : map_field ! (regs . tsd2) , tsd3 : map_field ! (regs . tsd3) , tx_fields : & mut self . tx_fields , } ,)) } else { None } } fn transmit (& mut self , _ : smoltcp :: time :: Instant) -> Option < TxToken < '_ > > { if self . tx_fields . remaining_bufs > 0 { let regs = self . regs . as_mut_ptr () ; Some (TxToken { tsd0 : map_field ! (regs . tsd0) , tsd1 : map_field ! (regs . tsd1) , tsd2 : map_field ! (regs . tsd2) , tsd3 : map_field ! (regs . tsd3) , tx_fields : & mut self . tx_fields , }) } else { None } } fn capabilities (& self) -> smoltcp :: phy :: DeviceCapabilities { let mut device_capabilities = DeviceCapabilities :: default () ; device_capabilities . medium = smoltcp :: phy :: Medium :: Ethernet ; device_capabilities . max_transmission_unit = usize :: from (self . mtu) ; device_capabilities . max_burst_size = Some (usize :: min (NO_TX_BUFFERS , RX_BUF_LEN / usize :: from (self . mtu) ,)) ; device_capabilities } }
};
}
