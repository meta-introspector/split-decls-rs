macro_rules! deps {
    () => {
        Sealed!();
        Error!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Sealed for String { fn symbol_name < R > (mut self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { if crate :: util :: check_null_bytes (self . as_bytes ()) ? { function (self . as_ptr () . cast ()) } else { self . push ('\0') ; function (self . as_ptr () . cast ()) } } }
    };
}

impl_19!()