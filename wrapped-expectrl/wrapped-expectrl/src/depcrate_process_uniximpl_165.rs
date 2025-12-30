// Generated macro for impl_165 (impl)
macro_rules! Depcrate_process_uniximpl_165 {
() => {
// Module: crate::process::unix
// Provides: {"impl_165"}
// Dependencies: {}
impl NonBlocking for PtyStream { fn set_blocking (& mut self , on : bool) -> Result < () > { let fd = self . handle . as_raw_fd () ; match on { true => make_non_blocking (fd , false) , false => make_non_blocking (fd , true) , } } }
};
}
