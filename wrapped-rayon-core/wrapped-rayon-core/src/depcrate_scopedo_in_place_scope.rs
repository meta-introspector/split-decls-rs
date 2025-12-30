// Generated macro for do_in_place_scope (function)
macro_rules! Depcrate_scopedo_in_place_scope {
() => {
// Module: crate::scope
// Provides: {"do_in_place_scope"}
// Dependencies: {}
pub (crate) fn do_in_place_scope < 'scope , OP , R > (registry : Option < & Arc < Registry > > , op : OP) -> R where OP : FnOnce (& Scope < 'scope >) -> R , { let (thread , registry) = get_in_place_thread_registry (registry) ; let scope = Scope :: < 'scope > :: new (thread , registry) ; scope . base . complete (thread , | | op (& scope)) }
};
}
