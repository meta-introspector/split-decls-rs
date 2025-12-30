// Generated macro for __sel_inner (macro)
macro_rules! Depcrate___macros_sel__sel_inner {
() => {
// Module: crate::__macros::sel
// Provides: {"__sel_inner"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (all (feature = "unstable-static-sel" , not (feature = "unstable-darwin-objc") , feature = "unstable-static-sel-inlined" ,))] macro_rules ! __sel_inner { ($ data : expr , $ hash : expr) => { { $ crate :: __statics_sel ! { ($ data) ($ hash) } # [allow (unused_unsafe)] unsafe { * REF . get () } } } ; }
};
}
