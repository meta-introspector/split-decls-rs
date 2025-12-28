macro_rules! deps {
    () => {
        Error!();
        Sealed!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Sealed for & String { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . symbol_name (function) } }
    };
}

impl_17!()