// Generated macro for refspec (module)
macro_rules! Depcrate_configrefspec {
() => {
// Module: crate::config
// Provides: {"refspec"}
// Dependencies: {}
# [doc = ""] pub mod refspec { # [doc = " The error produced when failing to parse a refspec from the configuration."] pub type Error = super :: key :: Error < gix_refspec :: parse :: Error , 'r' , 'p' > ; }
};
}
