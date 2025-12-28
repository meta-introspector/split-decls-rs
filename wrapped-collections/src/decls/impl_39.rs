macro_rules! deps {
    () => {
        IMap!();
        IIterator!();
        IIterable!();
        IMapView!();
        IKeyValuePair!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IMap < K , V > { pub fn Lookup < P0 > (& self , key : P0) -> windows_core :: Result < V > where P0 : windows_core :: Param < K > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Lookup) (windows_core :: Interface :: as_raw (this) , key . param () . abi () , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn Size (& self) -> windows_core :: Result < u32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Size) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn HasKey < P0 > (& self , key : P0) -> windows_core :: Result < bool > where P0 : windows_core :: Param < K > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . HasKey) (windows_core :: Interface :: as_raw (this) , key . param () . abi () , & mut result__ ,) . map (| | result__) } } pub fn GetView (& self) -> windows_core :: Result < IMapView < K , V > > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . GetView) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn Insert < P0 , P1 > (& self , key : P0 , value : P1) -> windows_core :: Result < bool > where P0 : windows_core :: Param < K > , P1 : windows_core :: Param < V > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Insert) (windows_core :: Interface :: as_raw (this) , key . param () . abi () , value . param () . abi () , & mut result__ ,) . map (| | result__) } } pub fn Remove < P0 > (& self , key : P0) -> windows_core :: Result < () > where P0 : windows_core :: Param < K > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Remove) (windows_core :: Interface :: as_raw (this) , key . param () . abi () ,) . ok () } } pub fn Clear (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Clear) (windows_core :: Interface :: as_raw (this)) . ok () } } pub fn First (& self) -> windows_core :: Result < IIterator < IKeyValuePair < K , V > > > { let this = & windows_core :: Interface :: cast :: < IIterable < IKeyValuePair < K , V > > > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . First) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
    };
}

impl_39!();