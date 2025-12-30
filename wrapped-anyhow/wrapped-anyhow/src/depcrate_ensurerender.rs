// Generated macro for render (function)
macro_rules! Depcrate_ensurerender {
() => {
// Module: crate::ensure
// Provides: {"render"}
// Dependencies: {}
fn render (msg : & 'static str , lhs : & dyn Debug , rhs : & dyn Debug) -> Error { let mut lhs_buf = Buf :: new () ; if fmt :: write (& mut lhs_buf , format_args ! ("{:?}" , lhs)) . is_ok () { let mut rhs_buf = Buf :: new () ; if fmt :: write (& mut rhs_buf , format_args ! ("{:?}" , rhs)) . is_ok () { let lhs_str = lhs_buf . as_str () ; let rhs_str = rhs_buf . as_str () ; let len = msg . len () + 2 + lhs_str . len () + 4 + rhs_str . len () + 1 ; let mut string = String :: with_capacity (len) ; string . push_str (msg) ; string . push_str (" (") ; string . push_str (lhs_str) ; string . push_str (" vs ") ; string . push_str (rhs_str) ; string . push (')') ; return Error :: msg (string) ; } } Error :: msg (msg) }
};
}
