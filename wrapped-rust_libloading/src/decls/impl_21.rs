macro_rules! deps {
    () => {
        Sealed!();
        Error!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Sealed for & CStr { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { function (self . as_ptr ()) } }
    };
}

impl_21!()