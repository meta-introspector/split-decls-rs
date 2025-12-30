// Generated macro for spawn_send (macro)
macro_rules! Depcrate_spawn_testspawn_send {
() => {
// Module: crate::spawn::test
// Provides: {"spawn_send"}
// Dependencies: {}
macro_rules ! spawn_send { ($ spawn : ident , $ tx : ident , $ i : expr) => { { let tx = $ tx . clone () ; $ spawn (move || tx . send ($ i) . unwrap ()) ; } } ; }
};
}
