// Generated macro for impl_55 (impl)
macro_rules! Depcrate_bindingsimpl_55 {
() => {
// Module: crate::bindings
// Provides: {"impl_55"}
// Dependencies: {}
impl ITest { pub fn Numerics (& self , n : Vector2) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Numerics) (windows_core :: Interface :: as_raw (this) , n ,) . ok () } } pub fn Collections < P0 > (& self , c : P0) -> windows_core :: Result < () > where P0 : windows_core :: Param < IVector < i32 > > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Collections) (windows_core :: Interface :: as_raw (this) , c . param () . abi () ,) . ok () } } pub fn Async (& self) -> windows_core :: Result < IAsyncAction > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Async) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn Windows < P0 > (& self , s : P0) -> windows_core :: Result < () > where P0 : windows_core :: Param < IStringable > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Windows) (windows_core :: Interface :: as_raw (this) , s . param () . abi () ,) . ok () } } }
};
}
