// Generated macro for IoDevice (enum)
macro_rules! Depcrate_consoleIoDevice {
() => {
// Module: crate::console
// Provides: {"IoDevice"}
// Dependencies: {}
pub (crate) enum IoDevice { # [cfg (not (target_arch = "riscv64"))] Uhyve (UhyveSerial) , Uart (SerialDevice) , # [cfg (feature = "console")] Virtio (VirtioUART) , }
};
}
