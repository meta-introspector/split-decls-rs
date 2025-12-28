macro_rules! IAsyncOperation {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IAsyncOperation < TResult > (windows_core :: IUnknown , core :: marker :: PhantomData < TResult >) where TResult : windows_core :: RuntimeType + 'static ;
    };
}

IAsyncOperation!();