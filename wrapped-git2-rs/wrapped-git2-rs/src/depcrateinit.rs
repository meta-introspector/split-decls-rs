// Generated macro for init (function)
macro_rules! Depcrateinit {
() => {
// Module: crate
// Provides: {"init"}
// Dependencies: {}
fn init () { static INIT : Once = Once :: new () ; INIT . call_once (| | { openssl_env_init () ; }) ; raw :: init () ; }
};
}
