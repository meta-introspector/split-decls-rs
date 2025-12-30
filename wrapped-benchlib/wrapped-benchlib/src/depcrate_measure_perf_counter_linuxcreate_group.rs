// Generated macro for create_group (function)
macro_rules! Depcrate_measure_perf_counter_linuxcreate_group {
() => {
// Module: crate::measure::perf_counter::linux
// Provides: {"create_group"}
// Dependencies: {}
fn create_group () -> anyhow :: Result < Group > { match Group :: new () { Ok (group) => Ok (group) , Err (error) => { let path = "/proc/sys/kernel/perf_event_paranoid" ; let level = std :: fs :: read_to_string (path) . unwrap_or_else (| _ | "unknown" . to_string ()) ; let level = level . trim () ; Err (anyhow :: anyhow ! ("Cannot create perf_event group ({:?}). Current value of {} is {}.
Try lowering it with `sudo bash -c 'echo -1 > /proc/sys/kernel/perf_event_paranoid'`." , error , path , level)) } } }
};
}
