// Generated macro for impl_356 (impl)
macro_rules! Depcrate_interface_array_returnimpl_356 {
() => {
// Module: crate::interface_array_return
// Provides: {"impl_356"}
// Dependencies: {}
impl IDispatch { pub unsafe fn GetTypeInfoCount (& self) -> windows_core :: Result < u32 > { unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (self) . GetTypeInfoCount) (windows_core :: Interface :: as_raw (self) , & mut result__ ,) . map (| | result__) } } pub unsafe fn GetIDsOfNames (& self , riid : * const windows_core :: GUID , rgsznames : * const windows_core :: PCWSTR , cnames : u32 , lcid : u32 , rgdispid : * mut i32 ,) -> windows_core :: Result < () > { unsafe { (windows_core :: Interface :: vtable (self) . GetIDsOfNames) (windows_core :: Interface :: as_raw (self) , riid , rgsznames , cnames , lcid , rgdispid as _ ,) . ok () } } }
};
}
