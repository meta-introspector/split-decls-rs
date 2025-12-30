// Generated macro for log (module)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
# [cfg (all (not (feature = "logging") , not (all (debug_assertions , feature = "alloc" , not (target_os = "none") ,))))] # [doc (hidden)] pub mod log { # [macro_export] macro_rules ! _internal_noop_log { ($ ($ t : expr) ,*) => { } ; } pub use crate :: _internal_noop_log as error ; pub use crate :: _internal_noop_log as warn ; pub use crate :: _internal_noop_log as info ; pub use crate :: _internal_noop_log as debug ; pub use crate :: _internal_noop_log as trace ; }
};
}
