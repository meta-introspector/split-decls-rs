macro_rules! deps {
    () => {
        Input!();
        Enum!();
        Struct!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl Input < '_ > { pub (crate) fn validate (& self) -> Result < () > { match self { Input :: Struct (input) => input . validate () , Input :: Enum (input) => input . validate () , } } }
    };
}

impl_113!();