macro_rules! IMapView {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IMapView < K , V > (windows_core :: IUnknown , core :: marker :: PhantomData < K > , core :: marker :: PhantomData < V > ,) where K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static ;
    };
}

IMapView!()