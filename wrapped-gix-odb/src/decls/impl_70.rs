macro_rules! deps {
    () => {
        Handle!();
        Store!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl super :: Handle < Arc < super :: Store > > { # [doc = " Convert a ref counted store into one that is ref-counted and thread-safe, by creating a new Store"] pub fn into_arc (self) -> std :: io :: Result < super :: Handle < Arc < super :: Store > > > { Ok (self) } }
    };
}

impl_70!()