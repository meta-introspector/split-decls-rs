macro_rules! deps {
    () => {
        Error!();
        Sealed!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Sealed for & CString { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { function (self . as_ptr ()) } }
    };
}

impl_23!();