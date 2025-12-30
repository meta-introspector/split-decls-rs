// Generated macro for print_memory_usage (function)
macro_rules! Depcrate_cliprint_memory_usage {
() => {
// Module: crate::cli
// Provides: {"print_memory_usage"}
// Dependencies: {}
fn print_memory_usage (mut host : AnalysisHost , vfs : Vfs) { let mem = host . per_query_memory_usage () ; let before = profile :: memory_usage () ; drop (vfs) ; let vfs = before . allocated - profile :: memory_usage () . allocated ; let before = profile :: memory_usage () ; drop (host) ; let unaccounted = before . allocated - profile :: memory_usage () . allocated ; let remaining = profile :: memory_usage () . allocated ; for (name , bytes , entries) in mem { eprintln ! ("{bytes:>8} {entries:>6} {name}") ; } eprintln ! ("{vfs:>8}        VFS") ; eprintln ! ("{unaccounted:>8}        Unaccounted") ; eprintln ! ("{remaining:>8}        Remaining") ; }
};
}
