macro_rules! impl_128 {
    () => {
        impl IAsyncAction { # [doc = " Calls `op(result)` when the `IAsyncAction` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < () >) + Send + 'static , { Async :: when (self , op) } }
    };
}

impl_128!()