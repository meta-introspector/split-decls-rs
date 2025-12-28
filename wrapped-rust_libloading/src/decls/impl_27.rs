macro_rules! deps {
    () => {
        Error!();
        Sealed!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Sealed for & [u8] { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { if crate :: util :: check_null_bytes (self) ? { function (self . as_ptr () . cast ()) } else { let copy = crate :: util :: copy_and_push (self , 0) ; function (copy . as_ptr () . cast ()) } } }
    };
}

impl_27!()