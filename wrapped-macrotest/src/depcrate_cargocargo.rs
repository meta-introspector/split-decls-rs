// Generated macro for cargo (function)
macro_rules! Depcrate_cargocargo {
() => {
// Module: crate::cargo
// Provides: {"cargo"}
// Dependencies: {}
fn cargo (project : & Project) -> Command { let mut cmd = raw_cargo () ; cmd . current_dir (& project . dir) ; cmd . env ("CARGO_TARGET_DIR" , & project . inner_target_dir) ; rustflags :: set_env (& mut cmd) ; cmd }
};
}
