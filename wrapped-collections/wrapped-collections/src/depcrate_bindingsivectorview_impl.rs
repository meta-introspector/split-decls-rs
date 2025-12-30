// Generated macro for IVectorView_Impl (trait)
macro_rules! Depcrate_bindingsIVectorView_Impl {
() => {
// Module: crate::bindings
// Provides: {"IVectorView_Impl"}
// Dependencies: {}
pub trait IVectorView_Impl < T > : IIterable_Impl < T > where T : windows_core :: RuntimeType + 'static , { fn GetAt (& self , index : u32) -> windows_core :: Result < T > ; fn Size (& self) -> windows_core :: Result < u32 > ; fn IndexOf (& self , value : windows_core :: Ref < T > , index : & mut u32) -> windows_core :: Result < bool > ; fn GetMany (& self , startIndex : u32 , items : & mut [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < u32 > ; }
};
}
