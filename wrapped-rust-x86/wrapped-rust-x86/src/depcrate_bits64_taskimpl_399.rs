// Generated macro for impl_399 (impl)
macro_rules! Depcrate_bits64_taskimpl_399 {
() => {
// Module: crate::bits64::task
// Provides: {"impl_399"}
// Dependencies: {}
impl TaskStateSegment { # [doc = " Creates a new empty TSS."] pub const fn new () -> TaskStateSegment { TaskStateSegment { reserved : 0 , rsp : [0 ; 3] , reserved2 : 0 , ist : [0 ; 7] , reserved3 : 0 , reserved4 : 0 , iomap_base : 0 , } } # [doc = " Sets the stack pointer (`stack_ptr`) to be used for when"] # [doc = " an interrupt causes the CPU to change RPL to `pl`."] pub fn set_rsp (& mut self , pl : Ring , stack_ptr : u64) { match pl { Ring :: Ring0 => self . rsp [0] = stack_ptr , Ring :: Ring1 => self . rsp [1] = stack_ptr , Ring :: Ring2 => self . rsp [2] = stack_ptr , Ring :: Ring3 => unreachable ! ("Can't set stack for PL3") , } } # [doc = " Sets the stack pointer (`stack_ptr`) to be used when"] # [doc = " an interrupt with a corresponding IST entry in the Interrupt"] # [doc = " Descriptor table pointing to the given `index` is raised."] pub fn set_ist (& mut self , index : usize , stack_ptr : u64) { match index { 0 => self . ist [0] = stack_ptr , 1 => self . ist [1] = stack_ptr , 2 => self . ist [2] = stack_ptr , 3 => self . ist [3] = stack_ptr , 4 => self . ist [4] = stack_ptr , 5 => self . ist [5] = stack_ptr , 6 => self . ist [6] = stack_ptr , _ => unreachable ! ("Can't set IST for this index (out of bounds).") , } } }
};
}
