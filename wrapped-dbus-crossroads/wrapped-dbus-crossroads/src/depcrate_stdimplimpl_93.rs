// Generated macro for impl_93 (impl)
macro_rules! Depcrate_stdimplimpl_93 {
() => {
// Module: crate::stdimpl
// Provides: {"impl_93"}
// Dependencies: {}
impl PropAllCtx { fn check_finished (& mut self) -> bool { if self . remaining > 0 { return false ; } if let Some (donefn) = self . donefn . take () { (donefn . 0) (self) ; } true } fn add_reply (& mut self , prop_name : String , prop_value : Option < Box < dyn RefArg + Send > >) { if let Some (v) = prop_value { self . answers . insert (prop_name , Variant (v)) ; } self . remaining -= 1 ; self . check_finished () ; } }
};
}
