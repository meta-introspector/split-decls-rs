// Generated macro for impl_142 (impl)
macro_rules! Depcrate_registerimpl_142 {
() => {
// Module: crate::register
// Provides: {"impl_142"}
// Dependencies: {}
impl Probe { pub (crate) const COUNT : usize = 256 ; # [doc = " Create a new probe with no features enabled."] pub fn new () -> Probe { Probe (ProbeAndOps (sys :: io_uring_probe :: default () , [sys :: io_uring_probe_op :: default () ; Probe :: COUNT] ,)) } # [inline] pub (crate) fn as_mut_ptr (& mut self) -> * mut sys :: io_uring_probe { & mut (self . 0) . 0 } # [doc = " Get whether a specific opcode is supported."] pub fn is_supported (& self , opcode : u8) -> bool { unsafe { let probe = & (self . 0) . 0 ; if opcode <= probe . last_op { let ops = probe . ops . as_slice (Self :: COUNT) ; ops [opcode as usize] . flags & (sys :: IO_URING_OP_SUPPORTED as u16) != 0 } else { false } } } }
};
}
