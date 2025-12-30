// Generated macro for infoentry (macro)
macro_rules! Depcrate_logginginfoentry {
() => {
// Module: crate::logging
// Provides: {"infoentry"}
// Dependencies: {}
# [cfg_attr (target_arch = "riscv64" , allow (unused))] macro_rules ! infoentry { ($ str : expr , $ rhs : expr) => (infoentry ! ($ str , "{}" , $ rhs)) ; ($ str : expr , $ ($ arg : tt) +) => (:: log :: info ! ("{:25}{}" , concat ! ($ str , ":") , format_args ! ($ ($ arg) +))) ; }
};
}
