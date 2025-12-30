// Generated macro for impl_222 (impl)
macro_rules! Depcrate_drivers_net_gemimpl_222 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"impl_222"}
// Dependencies: {}
impl TxFields { fn handle_interrupt (& mut self) { let int_status = unsafe { (* self . gem) . int_status . extract () } ; let receive_status = unsafe { (* self . gem) . receive_status . extract () } ; let transmit_status = unsafe { (* self . gem) . transmit_status . extract () } ; debug ! ("handle_interrupt\nint_status: {int_status:?}\nreceive_status: {receive_status:?}\ntransmit_status: {transmit_status:?}") ; if transmit_status . is_set (TransmitStatus :: TXCOMPL) { debug ! ("TX COMPLETE") ; unsafe { (* self . gem) . int_status . modify_no_read (int_status , Interrupts :: TXCOMPL :: SET) ; (* self . gem) . transmit_status . modify_no_read (transmit_status , TransmitStatus :: TXCOMPL :: SET) ; (* self . gem) . network_control . modify (NetworkControl :: TXEN :: CLEAR) ; } } let ret = int_status . is_set (Interrupts :: FRAMERX) && receive_status . is_set (ReceiveStatus :: FRAMERX) ; if ret { debug ! ("RX COMPLETE") ; unsafe { (* self . gem) . int_status . modify_no_read (int_status , Interrupts :: FRAMERX :: SET) ; (* self . gem) . receive_status . modify_no_read (receive_status , ReceiveStatus :: FRAMERX :: SET) ; } warn ! ("Interrupt is received but 'user space' will not be notified.") ; } } }
};
}
