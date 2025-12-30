// Generated macro for VsockDevCfgRaw (struct)
macro_rules! Depcrate_drivers_vsock_pciVsockDevCfgRaw {
() => {
// Module: crate::drivers::vsock::pci
// Provides: {"VsockDevCfgRaw"}
// Dependencies: {}
# [doc = " Virtio's socket device configuration structure."] # [doc = " See specification v1.1. - 5.11.4"] # [doc = ""] # [derive (Debug , Copy , Clone)] # [repr (C)] pub (crate) struct VsockDevCfgRaw { # [doc = " The guest_cid field contains the guest’s context ID, which uniquely identifies the device"] # [doc = " for its lifetime. The upper 32 bits of the CID are reserved and zeroed."] pub guest_cid : u64 , }
};
}
