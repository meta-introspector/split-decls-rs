// Generated macro for write_arg (function)
macro_rules! Depcrate_writewrite_arg {
() => {
// Module: crate::write
// Provides: {"write_arg"}
// Dependencies: {}
fn write_arg (w : & mut dyn Write , func : & Function , arg : Value) -> fmt :: Result { let ty = func . dfg . value_type (arg) ; if let Some (f) = & func . dfg . facts [arg] { write ! (w , "{arg} ! {f}: {ty}") } else { write ! (w , "{arg}: {ty}") } }
};
}
