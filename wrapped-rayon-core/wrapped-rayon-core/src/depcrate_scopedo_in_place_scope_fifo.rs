// Generated macro for do_in_place_scope_fifo (function)
macro_rules! Depcrate_scopedo_in_place_scope_fifo {
() => {
// Module: crate::scope
// Provides: {"do_in_place_scope_fifo"}
// Dependencies: {}
pub (crate) fn do_in_place_scope_fifo < 'scope , OP , R > (registry : Option < & Arc < Registry > > , op : OP) -> R where OP : FnOnce (& ScopeFifo < 'scope >) -> R , { let (thread , registry) = get_in_place_thread_registry (registry) ; let scope = ScopeFifo :: < 'scope > :: new (thread , registry) ; scope . base . complete (thread , | | op (& scope)) }
};
}
