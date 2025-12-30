// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn Error + Send + Sync > > { let opt = Opt :: parse () ; let data = ProfilingData :: new (& opt . file_prefix) ? ; if let Some (global_start_time) = data . iter () . filter_map (| e | e . start ()) . min () { for event in data . iter () { if let Some (thread_id) = opt . thread_id { if event . thread_id != thread_id { continue ; } } print_event (& data . to_full_event (& event) , global_start_time) ; } } else { eprintln ! ("No events.") ; } Ok (()) }
};
}
