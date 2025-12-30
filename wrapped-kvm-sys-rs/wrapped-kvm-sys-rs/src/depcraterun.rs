// Generated macro for Run (struct)
macro_rules! DepcrateRun {
() => {
// Module: crate
// Provides: {"Run"}
// Dependencies: {}
# [doc = " Information about the reason `run` returned"] # [allow (missing_docs)] # [repr (C)] # [derive (Copy)] pub struct Run { request_interrupt_window : u8 , padding1 : [u8 ; 7usize] , pub exit_reason : Exit , pub ready_for_interrupt_injection : u8 , pub if_flag : u8 , pub flags : u16 , pub cr8 : u64 , pub apic_base : u64 , _bindgen_data_1_ : [u64 ; 32usize] , pub kvm_valid_regs : u64 , pub kvm_dirty_regs : u64 , pub s : Union_Unnamed26 , }
};
}
