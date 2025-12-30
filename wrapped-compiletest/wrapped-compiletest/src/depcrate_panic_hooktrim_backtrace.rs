// Generated macro for trim_backtrace (function)
macro_rules! Depcrate_panic_hooktrim_backtrace {
() => {
// Module: crate::panic_hook
// Provides: {"trim_backtrace"}
// Dependencies: {}
# [doc = " On stable, short backtraces are only available to the default panic hook,"] # [doc = " so if we want something similar we have to resort to string processing."] fn trim_backtrace (full_backtrace : String) -> String { if rust_backtrace_full () { return full_backtrace ; } let mut buf = String :: with_capacity (full_backtrace . len ()) ; let mut on = false ; let mut skip_next_at = false ; let mut lines = full_backtrace . lines () ; while let Some (line) = lines . next () { if mem :: replace (& mut skip_next_at , false) && line . trim_start () . starts_with ("at ") { continue ; } if line . contains ("__rust_end_short_backtrace") { on = true ; skip_next_at = true ; continue ; } if line . contains ("__rust_begin_short_backtrace") { on = false ; skip_next_at = true ; continue ; } if on { writeln ! (buf , "{line}") . unwrap () ; } } writeln ! (buf , "note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.") . unwrap () ; buf }
};
}
