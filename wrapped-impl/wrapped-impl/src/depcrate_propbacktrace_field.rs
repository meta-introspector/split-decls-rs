// Generated macro for backtrace_field (function)
macro_rules! Depcrate_propbacktrace_field {
() => {
// Module: crate::prop
// Provides: {"backtrace_field"}
// Dependencies: {}
fn backtrace_field < 'a , 'b > (fields : & 'a [Field < 'b >]) -> Option < & 'a Field < 'b > > { for field in fields { if field . attrs . backtrace . is_some () { return Some (field) ; } } for field in fields { if field . is_backtrace () { return Some (field) ; } } None }
};
}
