// Generated macro for impl_6 (impl)
macro_rules! Depcrate_bindingsimpl_6 {
() => {
// Module: crate::bindings
// Provides: {"impl_6"}
// Dependencies: {}
impl IAsyncAction { pub fn GetResults (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . GetResults) (windows_core :: Interface :: as_raw (this ,)) . ok () } } pub fn Id (& self) -> windows_core :: Result < u32 > { let this = & windows_core :: Interface :: cast :: < IAsyncInfo > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Id) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn ErrorCode (& self) -> windows_core :: Result < windows_core :: HRESULT > { let this = & windows_core :: Interface :: cast :: < IAsyncInfo > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . ErrorCode) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn Cancel (& self) -> windows_core :: Result < () > { let this = & windows_core :: Interface :: cast :: < IAsyncInfo > (self) ? ; unsafe { (windows_core :: Interface :: vtable (this) . Cancel) (windows_core :: Interface :: as_raw (this)) . ok () } } pub fn Close (& self) -> windows_core :: Result < () > { let this = & windows_core :: Interface :: cast :: < IAsyncInfo > (self) ? ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
