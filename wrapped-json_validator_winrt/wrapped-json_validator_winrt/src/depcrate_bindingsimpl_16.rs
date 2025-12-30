// Generated macro for impl_16 (impl)
macro_rules! Depcrate_bindingsimpl_16 {
() => {
// Module: crate::bindings
// Provides: {"impl_16"}
// Dependencies: {}
impl JsonValidator { pub fn Validate (& self , value : & windows_core :: HSTRING ,) -> windows_core :: Result < windows_core :: HSTRING > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Validate) (windows_core :: Interface :: as_raw (this) , core :: mem :: transmute_copy (value) , & mut result__ ,) . map (| | core :: mem :: transmute (result__)) } } pub fn CreateInstance (schema : & windows_core :: HSTRING) -> windows_core :: Result < JsonValidator > { Self :: IJsonValidatorFactory (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . CreateInstance) (windows_core :: Interface :: as_raw (this) , core :: mem :: transmute_copy (schema) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) }) } fn IJsonValidatorFactory < R , F : FnOnce (& IJsonValidatorFactory) -> windows_core :: Result < R > > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < JsonValidator , IJsonValidatorFactory > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } }
};
}
