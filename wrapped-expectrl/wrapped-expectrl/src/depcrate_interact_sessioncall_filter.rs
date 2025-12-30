// Generated macro for call_filter (function)
macro_rules! Depcrate_interact_sessioncall_filter {
() => {
// Module: crate::interact::session
// Provides: {"call_filter"}
// Dependencies: {}
fn call_filter < F > (filter : Option < F > , buf : & [u8]) -> Result < Cow < '_ , [u8] > , Error > where F : FnMut (& [u8]) -> Result < Cow < '_ , [u8] > , Error > , { match filter { Some (mut action) => (action) (buf) , None => Ok (Cow :: Borrowed (buf)) , } }
};
}
