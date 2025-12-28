macro_rules! deps {
    () => {
        Error!();
        ErrorExtensions!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < E : Display > ErrorExtensions for & E { fn extend (& self) -> Error { Error { message : self . to_string () , source : None , extensions : None , } } }
    };
}

impl_54!();