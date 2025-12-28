macro_rules! deps {
    () => {
        ReadyAction!();
        ReadyState!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl IAsyncAction { # [doc = " Creates an `IAsyncAction` that is immediately ready with a value."] pub fn ready (result : Result < () >) -> Self { ReadyAction (ReadyState :: new (result)) . into () } }
    };
}

impl_147!();