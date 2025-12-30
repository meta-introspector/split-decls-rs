// Generated macro for impl_385 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_385 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_385"}
// Dependencies: {}
impl RTL8139Driver { fn tx_handler (& mut self) { for i in 0 .. self . tx_fields . tx_in_use . len () { if self . tx_fields . tx_in_use [i] { let txstatus = match i { 0 => self . regs . as_ptr () . tsd0 () . read () . to_ne () , 1 => self . regs . as_ptr () . tsd1 () . read () . to_ne () , 2 => self . regs . as_ptr () . tsd2 () . read () . to_ne () , 3 => self . regs . as_ptr () . tsd3 () . read () . to_ne () , _ => unreachable ! () , } ; if (txstatus & (TSD_TABT | TSD_OWC)) > 0 { error ! ("RTL8139: major error") ; continue ; } if (txstatus & TSD_TUN) == TSD_TUN { error ! ("RTL8139: transmit underrun") ; } if (txstatus & TSD_TOK) == TSD_TOK { self . tx_fields . tx_in_use [i] = false ; self . tx_fields . remaining_bufs += 1 ; } } } } }
};
}
