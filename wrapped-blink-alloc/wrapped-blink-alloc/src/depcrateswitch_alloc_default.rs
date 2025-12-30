// Generated macro for switch_alloc_default (macro)
macro_rules! Depcrateswitch_alloc_default {
() => {
// Module: crate
// Provides: {"switch_alloc_default"}
// Dependencies: {}
macro_rules ! switch_alloc_default { ($ ($ args : tt) *) => { feature_switch ! { ("alloc" => with_default | without_default) ($ ($ args) *) } } ; }
};
}
