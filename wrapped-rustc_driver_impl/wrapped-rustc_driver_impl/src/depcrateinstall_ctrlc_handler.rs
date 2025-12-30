// Generated macro for install_ctrlc_handler (function)
macro_rules! Depcrateinstall_ctrlc_handler {
() => {
// Module: crate
// Provides: {"install_ctrlc_handler"}
// Dependencies: {}
# [doc = " Install our usual `ctrlc` handler, which sets [`rustc_const_eval::CTRL_C_RECEIVED`]."] # [doc = " Making this handler optional lets tools can install a different handler, if they wish."] pub fn install_ctrlc_handler () { # [cfg (all (not (miri) , not (target_family = "wasm")))] ctrlc :: set_handler (move | | { rustc_const_eval :: CTRL_C_RECEIVED . store (true , Ordering :: Relaxed) ; std :: thread :: sleep (std :: time :: Duration :: from_millis (100)) ; std :: process :: exit (1) ; }) . expect ("Unable to install ctrlc handler") ; }
};
}
