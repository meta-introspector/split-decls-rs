// Generated macro for IVector_Impl (trait)
macro_rules! Depcrate_interface_iterableIVector_Impl {
() => {
// Module: crate::interface_iterable
// Provides: {"IVector_Impl"}
// Dependencies: {}
pub trait IVector_Impl < T > : IIterable_Impl < T > where T : windows_core :: RuntimeType + 'static , { fn GetAt (& self , index : u32) -> windows_core :: Result < T > ; fn Size (& self) -> windows_core :: Result < u32 > ; fn IndexOf (& self , value : windows_core :: Ref < T > , index : & mut u32) -> windows_core :: Result < bool > ; fn SetAt (& self , index : u32 , value : windows_core :: Ref < T >) -> windows_core :: Result < () > ; fn InsertAt (& self , index : u32 , value : windows_core :: Ref < T >) -> windows_core :: Result < () > ; fn RemoveAt (& self , index : u32) -> windows_core :: Result < () > ; fn Append (& self , value : windows_core :: Ref < T >) -> windows_core :: Result < () > ; fn RemoveAtEnd (& self) -> windows_core :: Result < () > ; fn Clear (& self) -> windows_core :: Result < () > ; fn GetMany (& self , startIndex : u32 , items : & mut [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < u32 > ; fn ReplaceAll (& self , items : & [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < () > ; }
};
}
