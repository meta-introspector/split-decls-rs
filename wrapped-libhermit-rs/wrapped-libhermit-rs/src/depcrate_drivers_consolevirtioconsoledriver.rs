// Generated macro for VirtioConsoleDriver (struct)
macro_rules! Depcrate_drivers_consoleVirtioConsoleDriver {
() => {
// Module: crate::drivers::console
// Provides: {"VirtioConsoleDriver"}
// Dependencies: {}
pub (crate) struct VirtioConsoleDriver { pub (super) dev_cfg : ConsoleDevCfg , pub (super) com_cfg : ComCfg , pub (super) isr_stat : IsrStatus , pub (super) notif_cfg : NotifCfg , pub (super) irq : InterruptLine , pub (super) recv_vq : RxQueue , pub (super) send_vq : TxQueue , }
};
}
