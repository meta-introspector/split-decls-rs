// Generated macro for GEMDriver (struct)
macro_rules! Depcrate_drivers_net_gemGEMDriver {
() => {
// Module: crate::drivers::net::gem
// Provides: {"GEMDriver"}
// Dependencies: {}
# [doc = " GEM network driver struct."] # [doc = ""] # [doc = " Struct allows to control device queues and also"] # [doc = " the device itself."] pub struct GEMDriver { mtu : u16 , irq : u8 , mac : [u8 ; 6] , rx_counter : u32 , rx_fields : RxFields , tx_counter : u32 , tx_fields : TxFields , }
};
}
