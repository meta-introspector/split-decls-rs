// Generated macro for dealloc (function)
macro_rules! Depcrate_rt_allocdealloc {
() => {
// Module: crate::rt::alloc
// Provides: {"dealloc"}
// Dependencies: {}
# [doc = " Track a raw deallocation"] pub (crate) fn dealloc (ptr : * mut u8 , location : Location) { let allocation = rt :: execution (| execution | match execution . raw_allocations . remove (& (ptr as usize)) { Some (allocation) => { trace ! (state = ? allocation . state , ? ptr , % location , "dealloc") ; allocation } None => panic ! ("pointer not tracked") , } ,) ; drop (allocation) ; }
};
}
