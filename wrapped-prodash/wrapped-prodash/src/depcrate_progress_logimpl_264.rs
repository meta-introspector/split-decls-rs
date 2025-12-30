// Generated macro for impl_264 (impl)
macro_rules! Depcrate_progress_logimpl_264 {
() => {
// Module: crate::progress::log
// Provides: {"impl_264"}
// Dependencies: {}
impl Log { fn maybe_log (& self) { if self . current_level > self . max_level { return ; } let step = self . step () ; if self . trigger . swap (false , Ordering :: Relaxed) { match (self . max , & self . unit) { (max , Some (unit)) => log :: info ! ("{} → {}" , self . name , unit . display (step , max , None)) , (Some (max) , None) => log :: info ! ("{} → {} / {}" , self . name , step , max) , (None , None) => log :: info ! ("{} → {}" , self . name , step) , } } } }
};
}
