// Generated macro for impl_585 (impl)
macro_rules! Depcrate_exceptionimpl_585 {
() => {
// Module: crate::exception
// Provides: {"impl_585"}
// Dependencies: {}
impl fmt :: Display for Exception { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { autoreleasepool_leaking (| pool | { if let Some (true) = self . is_nsexception () { let reason = unsafe { self . reason () } ; if let Some (reason) = & reason { let reason = unsafe { nsstring_to_str (reason , pool) } ; return write ! (f , "{reason}") ; } } write ! (f , "unknown exception") }) } }
};
}
