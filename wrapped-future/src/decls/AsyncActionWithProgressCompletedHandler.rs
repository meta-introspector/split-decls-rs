macro_rules! AsyncActionWithProgressCompletedHandler {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct AsyncActionWithProgressCompletedHandler < TProgress > (windows_core :: IUnknown , core :: marker :: PhantomData < TProgress > ,) where TProgress : windows_core :: RuntimeType + 'static ;
    };
}

AsyncActionWithProgressCompletedHandler!();