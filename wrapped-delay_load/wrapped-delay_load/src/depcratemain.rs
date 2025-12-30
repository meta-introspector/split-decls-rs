// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { unsafe { if let Some (api) = delay_load :: < ShellMessageBoxW > (s ! ("shlwapi.dll") , s ! ("ShellMessageBoxW")) { api (0 , 0 , w ! ("Message") , w ! ("Sample") , 1) ; } else { println ! ("Can't find API") ; } } }
};
}
