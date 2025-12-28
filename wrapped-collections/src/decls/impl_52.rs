macro_rules! deps {
    () => {
        IIterator!();
        IIterable!();
        IKeyValuePair!();
        IMapView!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IMapView < K , V > { pub fn Lookup < P0 > (& self , key : P0) -> windows_core :: Result < V > where P0 : windows_core :: Param < K > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Lookup) (windows_core :: Interface :: as_raw (this) , key . param () . abi () , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn Size (& self) -> windows_core :: Result < u32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Size) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn HasKey < P0 > (& self , key : P0) -> windows_core :: Result < bool > where P0 : windows_core :: Param < K > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . HasKey) (windows_core :: Interface :: as_raw (this) , key . param () . abi () , & mut result__ ,) . map (| | result__) } } pub fn Split (& self , first : & mut Option < IMapView < K , V > > , second : & mut Option < IMapView < K , V > > ,) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Split) (windows_core :: Interface :: as_raw (this) , first as * mut _ as _ , second as * mut _ as _ ,) . ok () } } pub fn First (& self) -> windows_core :: Result < IIterator < IKeyValuePair < K , V > > > { let this = & windows_core :: Interface :: cast :: < IIterable < IKeyValuePair < K , V > > > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . First) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
    };
}

impl_52!()