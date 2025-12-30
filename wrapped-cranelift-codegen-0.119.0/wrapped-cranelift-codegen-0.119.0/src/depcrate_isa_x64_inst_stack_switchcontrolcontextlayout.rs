// Generated macro for ControlContextLayout (struct)
macro_rules! Depcrate_isa_x64_inst_stack_switchControlContextLayout {
() => {
// Module: crate::isa::x64::inst::stack_switch
// Provides: {"ControlContextLayout"}
// Dependencies: {}
# [doc = " The `stack_switch` instruction loads information about the stack to switch"] # [doc = " to and stores information about the current stack by receiving pointers to"] # [doc = " memory laid out as in the struct `ControlContext` below."] # [doc = ""] # [doc = " The instruction is only supported on x64 Linux at the moment."] # [doc = ""] # [doc = " ```"] # [doc = " #[repr(C)]"] # [doc = " pub struct ControlContext {"] # [doc = "     pub stack_pointer: *mut u8,"] # [doc = "     pub frame_pointer: *mut u8,"] # [doc = "     pub instruction_pointer: *mut u8,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Note that this layout is deliberately chosen to make frame pointer walking"] # [doc = " possible, if desired: The layout enables stack layouts where a"] # [doc = " `ControlContext` is part of a frame pointer chain, putting the frame pointer"] # [doc = " next to the corresponding IP."] # [doc = ""] # [doc = " We never actually interact with values of that type in Cranelift, but are"] # [doc = " only interested in its layout for the purposes of generating code."] # [allow (dead_code)] pub struct ControlContextLayout { pub size : usize , pub stack_pointer_offset : usize , pub frame_pointer_offset : usize , pub ip_offset : usize , }
};
}
