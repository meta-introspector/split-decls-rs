// Generated macro for ARM32_SHIM (const)
macro_rules! DepcrateARM32_SHIM {
() => {
// Module: crate
// Provides: {"ARM32_SHIM"}
// Dependencies: {}
# [doc = " 32-bit ARM is not supported by Microsoft so ARM types are not generated."] # [doc = " Therefore we need to inject a few types to make the bindings work."] const ARM32_SHIM : & str = r#"
#[cfg(target_arch = "arm")]
#[repr(C)]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [u8; 257],
    pub szSystemStatus: [u8; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
}
#[cfg(target_arch = "arm")]
pub enum CONTEXT {}
"# ;
};
}
