macro_rules! deps {
    () => {
        AllObjects!();
        Store!();
        Error!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl dynamic :: Store { # [doc = " Like [`Handle::iter()`][super::Handle::iter()], but accessible directly on the store."] pub fn iter (& self) -> Result < AllObjects , dynamic :: load_index :: Error > { AllObjects :: new (self) } }
    };
}

impl_26!();