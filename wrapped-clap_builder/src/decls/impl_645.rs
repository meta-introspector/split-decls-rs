macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl From < & '_ Id > for Id { fn from (id : & '_ Id) -> Self { id . clone () } }
    };
}

impl_645!();