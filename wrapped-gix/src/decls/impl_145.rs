macro_rules! deps {
    () => {
        Id!();
        Platform!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'repo > Id < 'repo > { # [doc = " Obtain a platform for traversing ancestors of this commit."] pub fn ancestors (& self) -> crate :: revision :: walk :: Platform < 'repo > { crate :: revision :: walk :: Platform :: new (Some (self . inner) , self . repo) } }
    };
}

impl_145!();