// Generated macro for probe_loop (macro)
macro_rules! Depcrate_hpack_tableprobe_loop {
() => {
// Module: crate::hpack::table
// Provides: {"probe_loop"}
// Dependencies: {}
macro_rules ! probe_loop { ($ probe_var : ident < $ len : expr , $ body : expr) => { debug_assert ! ($ len > 0) ; loop { if $ probe_var < $ len { $ body $ probe_var += 1 ; } else { $ probe_var = 0 ; } } } ; }
};
}
