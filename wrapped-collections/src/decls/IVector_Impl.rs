macro_rules! deps {
    () => {
        IIterable_Impl!();
        IVectorView!();
    };
}

macro_rules! IVector_Impl {
    () => {
        deps!();
        pub trait IVector_Impl < T > : IIterable_Impl < T > where T : windows_core :: RuntimeType + 'static , { fn GetAt (& self , index : u32) -> windows_core :: Result < T > ; fn Size (& self) -> windows_core :: Result < u32 > ; fn GetView (& self) -> windows_core :: Result < IVectorView < T > > ; fn IndexOf (& self , value : windows_core :: Ref < T > , index : & mut u32) -> windows_core :: Result < bool > ; fn SetAt (& self , index : u32 , value : windows_core :: Ref < T >) -> windows_core :: Result < () > ; fn InsertAt (& self , index : u32 , value : windows_core :: Ref < T >) -> windows_core :: Result < () > ; fn RemoveAt (& self , index : u32) -> windows_core :: Result < () > ; fn Append (& self , value : windows_core :: Ref < T >) -> windows_core :: Result < () > ; fn RemoveAtEnd (& self) -> windows_core :: Result < () > ; fn Clear (& self) -> windows_core :: Result < () > ; fn GetMany (& self , startIndex : u32 , items : & mut [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < u32 > ; fn ReplaceAll (& self , items : & [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < () > ; }
    };
}

IVector_Impl!()