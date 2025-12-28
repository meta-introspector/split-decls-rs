macro_rules! IIterable {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IIterable < T > (windows_core :: IUnknown , core :: marker :: PhantomData < T >) where T : windows_core :: RuntimeType + 'static ;
    };
}

IIterable!();