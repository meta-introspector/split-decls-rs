macro_rules! deps {
    () => {
        IKeyValuePair!();
        IIterable_Impl!();
        IMapView!();
    };
}

macro_rules! IMapView_Impl {
    () => {
        deps!();
        pub trait IMapView_Impl < K , V > : IIterable_Impl < IKeyValuePair < K , V > > where K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static , { fn Lookup (& self , key : windows_core :: Ref < K >) -> windows_core :: Result < V > ; fn Size (& self) -> windows_core :: Result < u32 > ; fn HasKey (& self , key : windows_core :: Ref < K >) -> windows_core :: Result < bool > ; fn Split (& self , first : windows_core :: OutRef < IMapView < K , V > > , second : windows_core :: OutRef < IMapView < K , V > > ,) -> windows_core :: Result < () > ; }
    };
}

IMapView_Impl!()