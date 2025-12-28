macro_rules! AsyncOperationCompletedHandler {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct AsyncOperationCompletedHandler < TResult > (windows_core :: IUnknown , core :: marker :: PhantomData < TResult > ,) where TResult : windows_core :: RuntimeType + 'static ;
    };
}

AsyncOperationCompletedHandler!()