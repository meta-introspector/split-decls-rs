// Generated macro for impl_5 (impl)
macro_rules! Depcrate_bindingsimpl_5 {
() => {
// Module: crate::bindings
// Provides: {"impl_5"}
// Dependencies: {}
impl ITest { pub fn TestIterable < P0 > (& self , collection : P0 , values : & [i32]) -> windows_core :: Result < () > where P0 : windows_core :: Param < windows_collections :: IIterable < i32 > > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . TestIterable) (windows_core :: Interface :: as_raw (this) , collection . param () . abi () , values . len () . try_into () . unwrap () , values . as_ptr () ,) . ok () } } pub fn GetIterable (& self , values : & [i32] ,) -> windows_core :: Result < windows_collections :: IIterable < i32 > > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . GetIterable) (windows_core :: Interface :: as_raw (this) , values . len () . try_into () . unwrap () , values . as_ptr () , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn GetMapView (& self , values : & [i32] ,) -> windows_core :: Result < windows_collections :: IMapView < i32 , windows_collections :: IVectorView < i32 > > , > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . GetMapView) (windows_core :: Interface :: as_raw (this) , values . len () . try_into () . unwrap () , values . as_ptr () , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
