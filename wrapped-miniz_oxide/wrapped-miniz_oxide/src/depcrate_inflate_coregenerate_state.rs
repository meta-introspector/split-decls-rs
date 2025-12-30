// Generated macro for generate_state (macro)
macro_rules! Depcrate_inflate_coregenerate_state {
() => {
// Module: crate::inflate::core
// Provides: {"generate_state"}
// Dependencies: {}
macro_rules ! generate_state { ($ state : ident , $ state_machine : tt , $ f : expr) => { loop { match $ f { Action :: None => continue , Action :: Jump (new_state) => { $ state = new_state ; continue $ state_machine ; } , Action :: End (result) => break $ state_machine result , } } } ; }
};
}
