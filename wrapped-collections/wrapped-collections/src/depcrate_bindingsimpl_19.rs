// Generated macro for impl_19 (impl)
macro_rules! Depcrate_bindingsimpl_19 {
() => {
// Module: crate::bindings
// Provides: {"impl_19"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IIterator < T > { pub fn Current (& self) -> windows_core :: Result < T > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Current) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn HasCurrent (& self) -> windows_core :: Result < bool > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . HasCurrent) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn MoveNext (& self) -> windows_core :: Result < bool > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . MoveNext) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn GetMany (& self , items : & mut [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < u32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . GetMany) (windows_core :: Interface :: as_raw (this) , items . len () . try_into () . unwrap () , core :: mem :: transmute_copy (& items) , & mut result__ ,) . map (| | result__) } } }
};
}
