// Generated macro for impl_1394 (impl)
macro_rules! Depcrate_utils_change_trackerimpl_1394 {
() => {
// Module: crate::utils::change_tracker
// Provides: {"impl_1394"}
// Dependencies: {}
impl Display for ChangeSeverity { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { ChangeSeverity :: Info => write ! (f , "INFO") , ChangeSeverity :: Warning => write ! (f , "WARNING") , } } }
};
}
