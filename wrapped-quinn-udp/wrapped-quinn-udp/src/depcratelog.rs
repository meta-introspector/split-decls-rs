// Generated macro for log (module)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
# [allow (unused_imports , unused_macros)] mod log { # [cfg (all (feature = "log" , not (feature = "tracing-log")))] pub (crate) use log :: { debug , error , info , trace , warn } ; # [cfg (feature = "tracing-log")] pub (crate) use tracing :: { debug , error , info , trace , warn } ; # [cfg (not (any (feature = "log" , feature = "tracing-log")))] mod no_op { macro_rules ! trace (($ ($ tt : tt) *) => { { } }) ; macro_rules ! debug (($ ($ tt : tt) *) => { { } }) ; macro_rules ! info (($ ($ tt : tt) *) => { { } }) ; macro_rules ! log_warn (($ ($ tt : tt) *) => { { } }) ; macro_rules ! error (($ ($ tt : tt) *) => { { } }) ; pub (crate) use { debug , error , info , log_warn as warn , trace } ; } # [cfg (not (any (feature = "log" , feature = "tracing-log")))] pub (crate) use no_op :: * ; }
};
}
