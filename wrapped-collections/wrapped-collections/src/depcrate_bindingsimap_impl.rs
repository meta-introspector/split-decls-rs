// Generated macro for IMap_Impl (trait)
macro_rules! Depcrate_bindingsIMap_Impl {
() => {
// Module: crate::bindings
// Provides: {"IMap_Impl"}
// Dependencies: {}
pub trait IMap_Impl < K , V > : IIterable_Impl < IKeyValuePair < K , V > > where K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static , { fn Lookup (& self , key : windows_core :: Ref < K >) -> windows_core :: Result < V > ; fn Size (& self) -> windows_core :: Result < u32 > ; fn HasKey (& self , key : windows_core :: Ref < K >) -> windows_core :: Result < bool > ; fn GetView (& self) -> windows_core :: Result < IMapView < K , V > > ; fn Insert (& self , key : windows_core :: Ref < K > , value : windows_core :: Ref < V > ,) -> windows_core :: Result < bool > ; fn Remove (& self , key : windows_core :: Ref < K >) -> windows_core :: Result < () > ; fn Clear (& self) -> windows_core :: Result < () > ; }
};
}
