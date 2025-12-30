// Generated macro for impl_202 (impl)
macro_rules! Depcrate_mach_portimpl_202 {
() => {
// Module: crate::mach_port
// Provides: {"impl_202"}
// Dependencies: {}
impl CFMachPort { pub fn create_runloop_source (& self , order : CFIndex) -> Result < CFRunLoopSource , () > { unsafe { let runloop_source_ref = CFMachPortCreateRunLoopSource (kCFAllocatorDefault , self . 0 , order) ; if runloop_source_ref . is_null () { Err (()) } else { Ok (CFRunLoopSource :: wrap_under_create_rule (runloop_source_ref)) } } } }
};
}
