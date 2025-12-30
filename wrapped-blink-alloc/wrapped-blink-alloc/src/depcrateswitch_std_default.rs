// Generated macro for switch_std_default (macro)
macro_rules! Depcrateswitch_std_default {
() => {
// Module: crate
// Provides: {"switch_std_default"}
// Dependencies: {}
macro_rules ! switch_std_default { ($ ($ args : tt) *) => { feature_switch ! { ("std" => with_default | without_default) ($ ($ args) *) } } ; }
};
}
