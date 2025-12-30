// Generated macro for checkout (module)
macro_rules! Depcrate_configcheckout {
() => {
// Module: crate::config
// Provides: {"checkout"}
// Dependencies: {}
# [doc = ""] pub mod checkout { # [doc = ""] pub mod workers { use crate :: config ; # [doc = " The error produced when failing to parse the `checkout.workers` key."] pub type Error = config :: key :: Error < gix_config :: value :: Error , 'n' , 'd' > ; } }
};
}
