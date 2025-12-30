// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl HelperState { fn lock (& self) -> MutexGuard < '_ , HelperInner > { self . lock . lock () . unwrap_or_else (| e | e . into_inner ()) } # [doc = " Executes `f` for each request for a token, where `f` is expected to"] # [doc = " block and then provide the original closure with a token once it's"] # [doc = " acquired."] # [doc = ""] # [doc = " This is an infinite loop until the helper thread is dropped, at which"] # [doc = " point everything should get interrupted."] fn for_each_request (& self , mut f : impl FnMut (& HelperState)) { let mut lock = self . lock () ; while ! lock . producer_done { if lock . requests == 0 { lock = self . cvar . wait (lock) . unwrap_or_else (| e | e . into_inner ()) ; continue ; } lock . requests -= 1 ; drop (lock) ; f (self) ; lock = self . lock () ; } lock . consumer_done = true ; self . cvar . notify_one () ; } }
};
}
