macro_rules! deps {
    () => {
        Sealed!();
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Sealed for & str { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_bytes () . symbol_name (function) } }
    };
}

impl_15!()