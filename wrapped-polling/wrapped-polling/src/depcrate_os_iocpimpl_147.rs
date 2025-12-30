// Generated macro for impl_147 (impl)
macro_rules! Depcrate_os_iocpimpl_147 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_147"}
// Dependencies: {}
impl Events { # [doc = " Creates an empty list of events."] pub fn with_capacity (cap : usize) -> Events { Events { packets : Vec :: with_capacity (cap) , completions : Vec :: with_capacity (cap) , } } # [doc = " Iterate over I/O events."] pub fn iter (& self) -> impl Iterator < Item = Event > + '_ { self . packets . iter () . copied () } # [doc = " Clear the list of events."] pub fn clear (& mut self) { self . packets . clear () ; } # [doc = " The capacity of the list of events."] pub fn capacity (& self) -> usize { self . packets . capacity () } }
};
}
