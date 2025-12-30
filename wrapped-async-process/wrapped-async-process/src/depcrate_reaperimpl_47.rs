// Generated macro for impl_47 (impl)
macro_rules! Depcrate_reaperimpl_47 {
() => {
// Module: crate::reaper
// Provides: {"impl_47"}
// Dependencies: {}
impl ChildGuard { # [doc = " Get a reference to the inner process."] pub (crate) fn get_mut (& mut self) -> & mut std :: process :: Child { cfg_wait ! { if let Self :: Wait (this) = self { return this . get_mut () ; } } cfg_signal ! { if let Self :: Signal (this) = self { return this . get_mut () ; } } unreachable ! () } # [doc = " Start reaping this child process."] pub (crate) fn reap (& mut self , reaper : & 'static Reaper) { cfg_wait ! { if let (Self :: Wait (this) , Reaper :: Wait (reaper)) = (& mut * self , reaper) { this . reap (reaper) ; return ; } } cfg_signal ! { if let (Self :: Signal (this) , Reaper :: Signal (reaper)) = (self , reaper) { this . reap (reaper) ; return ; } } unreachable ! () } }
};
}
