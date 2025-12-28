macro_rules! IIterator_Impl {
    () => {
        pub trait IIterator_Impl < T > : windows_core :: IUnknownImpl where T : windows_core :: RuntimeType + 'static , { fn Current (& self) -> windows_core :: Result < T > ; fn HasCurrent (& self) -> windows_core :: Result < bool > ; fn MoveNext (& self) -> windows_core :: Result < bool > ; fn GetMany (& self , items : & mut [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < u32 > ; }
    };
}

IIterator_Impl!()