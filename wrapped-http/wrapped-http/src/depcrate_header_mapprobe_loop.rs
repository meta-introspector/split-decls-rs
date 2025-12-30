// Generated macro for probe_loop (macro)
macro_rules! Depcrate_header_mapprobe_loop {
() => {
// Module: crate::header::map
// Provides: {"probe_loop"}
// Dependencies: {}
macro_rules ! probe_loop { ($ label : tt : $ probe_var : ident < $ len : expr , $ body : expr) => { debug_assert ! ($ len > 0) ; $ label : loop { if $ probe_var < $ len { $ body $ probe_var += 1 ; } else { $ probe_var = 0 ; } } } ; ($ probe_var : ident < $ len : expr , $ body : expr) => { debug_assert ! ($ len > 0) ; loop { if $ probe_var < $ len { $ body $ probe_var += 1 ; } else { $ probe_var = 0 ; } } } ; }
};
}
