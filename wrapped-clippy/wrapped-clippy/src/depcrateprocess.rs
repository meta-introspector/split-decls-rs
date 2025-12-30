// Generated macro for process (function)
macro_rules! Depcrateprocess {
() => {
// Module: crate
// Provides: {"process"}
// Dependencies: {}
fn process < I > (old_args : I) -> Result < () , i32 > where I : Iterator < Item = String > , { let cmd = ClippyCmd :: new (old_args) ; let mut cmd = cmd . into_std_cmd () ; let exit_status = cmd . spawn () . expect ("could not run cargo") . wait () . expect ("failed to wait for cargo?") ; if exit_status . success () { Ok (()) } else { Err (exit_status . code () . unwrap_or (- 1)) } }
};
}
