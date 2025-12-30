// Generated macro for impl_83 (impl)
macro_rules! Depcrate_driver_shutdownimpl_83 {
() => {
// Module: crate::driver::shutdown
// Provides: {"impl_83"}
// Dependencies: {}
# [doc = " Lifecycle"] impl State { # [doc = " Handle long-running processes according to `mode`. If an error occurs, all remaining processes will be ignored automatically."] # [doc = " Return a list of `(process, Option<status>)`"] pub fn shutdown (self , mode : Mode) -> Result < Vec < (BString , Option < std :: process :: ExitStatus >) > , std :: io :: Error > { let mut out = Vec :: with_capacity (self . running . len ()) ; for (cmd , client) in self . running { match mode { Mode :: WaitForProcesses => { let mut child = client . into_child () ; let status = child . wait () ? ; out . push ((cmd , Some (status))) ; } Mode :: Ignore => { out . push ((cmd , None)) ; } } } Ok (out) } }
};
}
