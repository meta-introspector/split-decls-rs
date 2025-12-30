// Generated macro for impl_73 (impl)
macro_rules! Depcrate_consoleimpl_73 {
() => {
// Module: crate::console
// Provides: {"impl_73"}
// Dependencies: {}
impl Console { pub fn new (device : IoDevice) -> Self { Self { device , buffer : Vec :: new () , } } # [cfg (feature = "console")] pub fn replace_device (& mut self , device : IoDevice) { self . device = device ; } }
};
}
