// Generated macro for NetworkDriver (trait)
macro_rules! Depcrate_drivers_netNetworkDriver {
() => {
// Module: crate::drivers::net
// Provides: {"NetworkDriver"}
// Dependencies: {}
# [doc = " A trait for accessing the network interface"] pub (crate) trait NetworkDriver : Driver + smoltcp :: phy :: Device { # [doc = " Returns the mac address of the device."] fn get_mac_address (& self) -> [u8 ; 6] ; # [doc = " Check if a packet is available"] # [allow (dead_code)] fn has_packet (& self) -> bool ; # [doc = " Enable / disable the polling mode of the network interface"] fn set_polling_mode (& mut self , value : bool) ; # [doc = " Handle interrupt and check if a packet is available"] # [allow (dead_code)] fn handle_interrupt (& mut self) ; }
};
}
