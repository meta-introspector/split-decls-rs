macro_rules! impl_116 {
    () => {
        impl IAsyncAction { # [doc = " Waits for the `IAsyncAction` to finish."] pub fn join (& self) -> Result < () > { Async :: join (self) } }
    };
}

impl_116!()