// Generated macro for set_handler_inner (function)
macro_rules! Depcrateset_handler_inner {
() => {
// Module: crate
// Provides: {"set_handler_inner"}
// Dependencies: {}
fn set_handler_inner < F > (mut user_handler : F , overwrite : bool) -> Result < () , Error > where F : FnMut () + 'static + Send , { unsafe { platform :: init_os_handler (overwrite) ? ; } thread :: Builder :: new () . name ("ctrl-c" . into ()) . spawn (move | | loop { unsafe { platform :: block_ctrl_c () . expect ("Critical system error while waiting for Ctrl-C") ; } user_handler () ; }) . map_err (Error :: System) ? ; Ok (()) }
};
}
