// Generated macro for INT_MASK (const)
macro_rules! Depcrate_drivers_net_rtl8139INT_MASK {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"INT_MASK"}
// Dependencies: {}
# [doc = " To set the RTL8139 to accept only the Transmit OK (TOK) and Receive OK (ROK)"] # [doc = " interrupts, we would have the TOK and ROK bits of the IMR high and leave the"] # [doc = " rest low. That way when a TOK or ROK IRQ happens, it actually will go through"] # [doc = " and fire up an IRQ."] const INT_MASK : u16 = ISR_ROK | ISR_TOK | ISR_RXOVW | ISR_TER | ISR_RER ;
};
}
