// Generated macro for impl_69 (impl)
macro_rules! Depcrate_configimpl_69 {
() => {
// Module: crate::config
// Provides: {"impl_69"}
// Dependencies: {}
impl MethodData { pub (crate) fn merge_with_superclass (self , superclass : Self) -> Self { let unsafe_ = match (self . unsafe_ , superclass . unsafe_) { (Some (unsafe_) , _) => Some (unsafe_) , (_ , Some (true)) => Some (true) , _ => None , } ; Self { unsafe_ , renamed : self . renamed . or (superclass . renamed) . clone () , skipped : self . skipped | superclass . skipped , arguments : self . arguments , return_ : self . return_ , } } }
};
}
