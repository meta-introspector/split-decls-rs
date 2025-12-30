// Generated macro for RTL8139Driver (struct)
macro_rules! Depcrate_drivers_net_rtl8139RTL8139Driver {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"RTL8139Driver"}
// Dependencies: {}
# [doc = " RealTek RTL8139 network driver struct."] # [doc = ""] # [doc = " Struct allows to control device queues as also"] # [doc = " the device itself."] pub (crate) struct RTL8139Driver { regs : VolatileRef < 'static , Regs > , mtu : u16 , irq : InterruptLine , mac : [u8 ; 6] , rx_fields : RxFields , tx_fields : TxFields , }
};
}
