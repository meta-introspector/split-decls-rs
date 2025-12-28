macro_rules! deps {
    () => {
        IIterator!();
        IIterable!();
        IVectorView!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > IVectorView < T > { pub fn GetAt (& self , index : u32) -> windows_core :: Result < T > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . GetAt) (windows_core :: Interface :: as_raw (this) , index , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn Size (& self) -> windows_core :: Result < u32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Size) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn IndexOf < P0 > (& self , value : P0 , index : & mut u32) -> windows_core :: Result < bool > where P0 : windows_core :: Param < T > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . IndexOf) (windows_core :: Interface :: as_raw (this) , value . param () . abi () , index , & mut result__ ,) . map (| | result__) } } pub fn GetMany (& self , startindex : u32 , items : & mut [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < u32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . GetMany) (windows_core :: Interface :: as_raw (this) , startindex , items . len () . try_into () . unwrap () , core :: mem :: transmute_copy (& items) , & mut result__ ,) . map (| | result__) } } pub fn First (& self) -> windows_core :: Result < IIterator < T > > { let this = & windows_core :: Interface :: cast :: < IIterable < T > > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . First) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
    };
}

impl_78!()