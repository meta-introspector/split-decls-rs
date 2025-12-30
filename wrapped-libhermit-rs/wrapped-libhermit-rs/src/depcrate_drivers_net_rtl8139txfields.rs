// Generated macro for TxFields (struct)
macro_rules! Depcrate_drivers_net_rtl8139TxFields {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"TxFields"}
// Dependencies: {}
struct TxFields { tx_in_use : [bool ; NO_TX_BUFFERS] , tx_counter : usize , txbuffer : Box < [u8] , DeviceAlloc > , remaining_bufs : usize , }
};
}
