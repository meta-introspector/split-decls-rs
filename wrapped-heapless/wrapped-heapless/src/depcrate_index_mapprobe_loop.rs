// Generated macro for probe_loop (macro)
macro_rules! Depcrate_index_mapprobe_loop {
() => {
// Module: crate::index_map
// Provides: {"probe_loop"}
// Dependencies: {}
macro_rules ! probe_loop { ($ probe_var : ident < $ len : expr , $ body : expr) => { loop { if $ probe_var < $ len { $ body $ probe_var += 1 ; } else { $ probe_var = 0 ; } } } }
};
}
