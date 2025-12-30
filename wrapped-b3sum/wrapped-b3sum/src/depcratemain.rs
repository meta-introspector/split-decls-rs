// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> anyhow :: Result < () > { let args = Args :: parse () ? ; let mut thread_pool_builder = rayon_core :: ThreadPoolBuilder :: new () ; if let Some (num_threads) = args . num_threads () { thread_pool_builder = thread_pool_builder . num_threads (num_threads) ; } let thread_pool = thread_pool_builder . build () ? ; thread_pool . install (| | { let mut files_failed = 0u64 ; for path in & args . file_args { if args . check () { check_one_checkfile (path , & args , & mut files_failed) ? ; } else { let result = hash_one_input (path , & args) ; if let Err (e) = result { files_failed = files_failed . saturating_add (1) ; eprintln ! ("{}: {}: {}" , NAME , path . to_string_lossy () , e) ; } } } if args . check () && files_failed > 0 { eprintln ! ("{}: WARNING: {} computed checksum{} did NOT match" , NAME , files_failed , if files_failed == 1 { "" } else { "s" } ,) ; } std :: process :: exit (if files_failed > 0 { 1 } else { 0 }) ; }) }
};
}
