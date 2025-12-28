macro_rules! deps {
    () => {
        BindIndex!();
        Statement!();
        Result!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl BindIndex for usize { # [inline] fn idx (& self , _ : & Statement < '_ >) -> Result < usize > { Ok (* self) } }
    };
}

impl_37!();