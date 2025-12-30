// Generated macro for impl_228 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_228 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_228"}
// Dependencies: {}
impl smoltcp :: phy :: Device for GEMDriver { # [doc = " The index of the free Rx buffer"] type RxToken < 'a > = RxToken < 'a > ; type TxToken < 'a > = TxToken < 'a > ; fn receive (& mut self , timestamp : smoltcp :: time :: Instant ,) -> Option < (Self :: RxToken < '_ > , Self :: TxToken < '_ >) > { if let Some (rx_index) = self . next_rx_index () && let Some (tx_index) = self . next_tx_index () { self . reserve_tx_index (tx_index) ; self . rx_counter = (rx_index + 1) % RX_BUF_NUM ; self . rx_fields . rxbuffer_reserved [usize :: try_from (rx_index) . unwrap ()] = true ; Some ((RxToken { buffer_index : rx_index , rx_fields : & mut self . rx_fields , } , TxToken { buffer_index : tx_index , tx_fields : & mut self . tx_fields , } ,)) } else { None } } fn transmit (& mut self , timestamp : smoltcp :: time :: Instant) -> Option < Self :: TxToken < '_ > > { self . handle_interrupt () ; self . next_tx_index () . inspect (| index | self . reserve_tx_index (* index)) . map (| buffer_index | TxToken { buffer_index , tx_fields : & mut self . tx_fields , }) } fn capabilities (& self) -> DeviceCapabilities { let mut cap = DeviceCapabilities :: default () ; cap . max_transmission_unit = usize :: from (self . mtu) ; cap . max_burst_size = Some (u32 :: max (TX_BUF_NUM , RX_BUF_NUM) . try_into () . unwrap ()) ; cap . checksum = ChecksumCapabilities :: default () ; cap } }
};
}
