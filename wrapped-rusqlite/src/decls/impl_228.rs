macro_rules! deps {
    () => {
        Statement!();
        RowIndex!();
        Result!();
        Error!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl RowIndex for usize { # [inline] fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { if * self >= stmt . column_count () { Err (Error :: InvalidColumnIndex (* self)) } else { Ok (* self) } } }
    };
}

impl_228!()