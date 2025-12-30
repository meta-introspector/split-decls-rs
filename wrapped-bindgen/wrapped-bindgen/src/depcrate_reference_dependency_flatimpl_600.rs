// Generated macro for impl_600 (impl)
macro_rules! Depcrate_reference_dependency_flatimpl_600 {
() => {
// Module: crate::reference_dependency_flat
// Provides: {"impl_600"}
// Dependencies: {}
impl IMemoryBufferReference { pub fn Capacity (& self) -> windows_core :: Result < u32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Capacity) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn RemoveClosed (& self , cookie : i64) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . RemoveClosed) (windows_core :: Interface :: as_raw (this) , cookie ,) . ok () } } pub fn Close (& self) -> windows_core :: Result < () > { let this = & windows_core :: Interface :: cast :: < IClosable > (self) ? ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
