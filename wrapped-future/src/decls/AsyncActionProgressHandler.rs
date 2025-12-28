macro_rules! AsyncActionProgressHandler {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct AsyncActionProgressHandler < TProgress > (windows_core :: IUnknown , core :: marker :: PhantomData < TProgress > ,) where TProgress : windows_core :: RuntimeType + 'static ;
    };
}

AsyncActionProgressHandler!()