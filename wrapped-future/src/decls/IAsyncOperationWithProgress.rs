macro_rules! IAsyncOperationWithProgress {
    () => {
        # [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IAsyncOperationWithProgress < TResult , TProgress > (windows_core :: IUnknown , core :: marker :: PhantomData < TResult > , core :: marker :: PhantomData < TProgress > ,) where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static ;
    };
}

IAsyncOperationWithProgress!();