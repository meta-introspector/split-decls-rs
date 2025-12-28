macro_rules! deps {
    () => {
        Error!();
        Sealed!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < const N : usize > Sealed for & [u8 ; N] { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_slice () . symbol_name (function) } }
    };
}

impl_31!()