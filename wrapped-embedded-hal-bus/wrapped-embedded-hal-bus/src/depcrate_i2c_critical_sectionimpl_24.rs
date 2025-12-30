// Generated macro for impl_24 (impl)
macro_rules! Depcrate_i2c_critical_sectionimpl_24 {
() => {
// Module: crate::i2c::critical_section
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , T > CriticalSectionDevice < 'a , T > { # [doc = " Create a new `CriticalSectionDevice`."] # [inline] pub fn new (bus : & 'a Mutex < RefCell < T > >) -> Self { Self { bus } } }
};
}
