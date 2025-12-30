// Generated macro for ITest_Impl (trait)
macro_rules! Depcrate_bindingsITest_Impl {
() => {
// Module: crate::bindings
// Provides: {"ITest_Impl"}
// Dependencies: {}
pub trait ITest_Impl : windows_core :: IUnknownImpl { fn TestIterable (& self , collection : windows_core :: Ref < windows_collections :: IIterable < i32 > > , values : & [i32] ,) -> windows_core :: Result < () > ; fn GetIterable (& self , values : & [i32] ,) -> windows_core :: Result < windows_collections :: IIterable < i32 > > ; fn GetMapView (& self , values : & [i32] ,) -> windows_core :: Result < windows_collections :: IMapView < i32 , windows_collections :: IVectorView < i32 > > , > ; }
};
}
