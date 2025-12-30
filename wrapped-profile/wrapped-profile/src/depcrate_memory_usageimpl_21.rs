// Generated macro for impl_21 (impl)
macro_rules! Depcrate_memory_usageimpl_21 {
() => {
// Module: crate::memory_usage
// Provides: {"impl_21"}
// Dependencies: {}
impl fmt :: Display for Bytes { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let bytes = self . 0 ; let mut value = bytes ; let mut suffix = "b" ; if value . abs () > 4096 { value /= 1024 ; suffix = "kb" ; if value . abs () > 4096 { value /= 1024 ; suffix = "mb" ; } } f . pad (& format ! ("{value}{suffix}")) } }
};
}
