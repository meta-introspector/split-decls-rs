// Generated macro for ro (module)
macro_rules! Depcrate_vmx_vmcsro {
() => {
// Module: crate::vmx::vmcs
// Provides: {"ro"}
// Dependencies: {}
# [doc = " VM-exit information fields."] pub mod ro { # [doc = " Guest-physical address (full)."] pub const GUEST_PHYSICAL_ADDR_FULL : u32 = 0x2400 ; # [doc = " Guest-physical address (high)."] pub const GUEST_PHYSICAL_ADDR_HIGH : u32 = 0x2401 ; # [doc = " VM-instruction error."] pub const VM_INSTRUCTION_ERROR : u32 = 0x4400 ; # [doc = " Exit reason."] pub const EXIT_REASON : u32 = 0x4402 ; # [doc = " VM-exit interruption information."] pub const VMEXIT_INTERRUPTION_INFO : u32 = 0x4404 ; # [doc = " VM-exit interruption error code."] pub const VMEXIT_INTERRUPTION_ERR_CODE : u32 = 0x4406 ; # [doc = " IDT-vectoring information field."] pub const IDT_VECTORING_INFO : u32 = 0x4408 ; # [doc = " IDT-vectoring error code."] pub const IDT_VECTORING_ERR_CODE : u32 = 0x440A ; # [doc = " VM-exit instruction length."] pub const VMEXIT_INSTRUCTION_LEN : u32 = 0x440C ; # [doc = " VM-exit instruction information."] pub const VMEXIT_INSTRUCTION_INFO : u32 = 0x440E ; # [doc = " Exit qualification."] pub const EXIT_QUALIFICATION : u32 = 0x6400 ; # [doc = " I/O RCX."] pub const IO_RCX : u32 = 0x6402 ; # [doc = " I/O RSI."] pub const IO_RSI : u32 = 0x6404 ; # [doc = " I/O RDI."] pub const IO_RDI : u32 = 0x6406 ; # [doc = " I/O RIP."] pub const IO_RIP : u32 = 0x6408 ; # [doc = " Guest-linear address."] pub const GUEST_LINEAR_ADDR : u32 = 0x640A ; }
};
}
