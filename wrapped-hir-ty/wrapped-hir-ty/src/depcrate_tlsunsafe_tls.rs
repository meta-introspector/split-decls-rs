// Generated macro for unsafe_tls (module)
macro_rules! Depcrate_tlsunsafe_tls {
() => {
// Module: crate::tls
// Provides: {"unsafe_tls"}
// Dependencies: {}
mod unsafe_tls { use super :: DebugContext ; use crate :: db :: HirDatabase ; use scoped_tls :: scoped_thread_local ; scoped_thread_local ! (static PROGRAM : DebugContext <'_ >) ; pub (crate) fn with_current_program < R > (op : impl for < 'a > FnOnce (Option < & 'a DebugContext < 'a > >) -> R ,) -> R { if PROGRAM . is_set () { PROGRAM . with (| prog | op (Some (prog))) } else { op (None) } } pub (crate) fn set_current_program < OP , R > (p : & dyn HirDatabase , op : OP) -> R where OP : FnOnce () -> R , { let ctx = DebugContext (p) ; let static_p : & DebugContext < 'static > = unsafe { std :: mem :: transmute :: < & DebugContext < '_ > , & DebugContext < 'static > > (& ctx) } ; PROGRAM . set (static_p , op) } }
};
}
