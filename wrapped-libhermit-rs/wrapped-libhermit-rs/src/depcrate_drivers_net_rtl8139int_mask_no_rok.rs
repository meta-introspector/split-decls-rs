// Generated macro for INT_MASK_NO_ROK (const)
macro_rules! Depcrate_drivers_net_rtl8139INT_MASK_NO_ROK {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"INT_MASK_NO_ROK"}
// Dependencies: {}
# [doc = " Beside Receive OK (ROK) interrupt, this mask enable all other interrupts"] const INT_MASK_NO_ROK : u16 = ISR_TOK | ISR_RXOVW | ISR_TER | ISR_RER ;
};
}
