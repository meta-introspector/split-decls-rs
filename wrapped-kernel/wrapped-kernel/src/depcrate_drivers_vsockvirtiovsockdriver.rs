// Generated macro for VirtioVsockDriver (struct)
macro_rules! Depcrate_drivers_vsockVirtioVsockDriver {
() => {
// Module: crate::drivers::vsock
// Provides: {"VirtioVsockDriver"}
// Dependencies: {}
pub (crate) struct VirtioVsockDriver { pub (super) dev_cfg : VsockDevCfg , pub (super) com_cfg : ComCfg , pub (super) isr_stat : IsrStatus , pub (super) notif_cfg : NotifCfg , pub (super) irq : InterruptLine , pub (super) event_vq : EventQueue , pub (super) recv_vq : RxQueue , pub (super) send_vq : TxQueue , }
};
}
