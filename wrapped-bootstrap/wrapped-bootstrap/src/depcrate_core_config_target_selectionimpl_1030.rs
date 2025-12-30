// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_core_config_target_selectionimpl_1030 {
() => {
// Module: crate::core::config::target_selection
// Provides: {"impl_1030"}
// Dependencies: {}
impl fmt :: Display for TargetSelection { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . triple) ? ; if let Some (file) = self . file { write ! (f , "({file})") ? ; } Ok (()) } }
};
}
