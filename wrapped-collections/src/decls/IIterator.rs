macro_rules! IIterator {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IIterator < T > (windows_core :: IUnknown , core :: marker :: PhantomData < T >) where T : windows_core :: RuntimeType + 'static ;
    };
}

IIterator!()