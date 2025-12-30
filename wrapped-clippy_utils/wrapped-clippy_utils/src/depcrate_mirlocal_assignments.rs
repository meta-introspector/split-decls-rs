// Generated macro for local_assignments (function)
macro_rules! Depcrate_mirlocal_assignments {
() => {
// Module: crate::mir
// Provides: {"local_assignments"}
// Dependencies: {}
# [doc = " Returns a vector of `mir::Location` where `local` is assigned."] pub fn local_assignments (mir : & Body < '_ > , local : Local) -> Vec < Location > { let mut locations = Vec :: new () ; for (block , data) in mir . basic_blocks . iter_enumerated () { for statement_index in 0 ..= data . statements . len () { let location = Location { block , statement_index } ; if is_local_assignment (mir , local , location) { locations . push (location) ; } } } locations }
};
}
