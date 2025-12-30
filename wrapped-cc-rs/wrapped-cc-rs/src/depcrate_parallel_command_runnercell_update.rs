// Generated macro for cell_update (function)
macro_rules! Depcrate_parallel_command_runnercell_update {
() => {
// Module: crate::parallel::command_runner
// Provides: {"cell_update"}
// Dependencies: {}
fn cell_update < T , F > (cell : & Cell < T > , f : F) where T : Default , F : FnOnce (T) -> T , { let old = cell . take () ; let new = f (old) ; cell . set (new) ; }
};
}
