macro_rules! IAsyncActionWithProgress {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IAsyncActionWithProgress < TProgress > (windows_core :: IUnknown , core :: marker :: PhantomData < TProgress > ,) where TProgress : windows_core :: RuntimeType + 'static ;
    };
}

IAsyncActionWithProgress!();