macro_rules! IVectorView {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IVectorView < T > (windows_core :: IUnknown , core :: marker :: PhantomData < T >) where T : windows_core :: RuntimeType + 'static ;
    };
}

IVectorView!()