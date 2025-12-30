// Generated macro for impl_37 (impl)
macro_rules! Depcrate_adapter_winconimpl_37 {
() => {
// Module: crate::adapter::wincon
// Provides: {"impl_37"}
// Dependencies: {}
impl Iterator for WinconBytesIter < '_ > { type Item = (anstyle :: Style , String) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_bytes (& mut self . bytes , self . parser , self . capture) } }
};
}
