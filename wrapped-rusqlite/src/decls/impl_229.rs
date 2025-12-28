macro_rules! deps {
    () => {
        RowIndex!();
        Statement!();
        Result!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl RowIndex for & '_ str { # [inline] fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { stmt . column_index (self) } }
    };
}

impl_229!()