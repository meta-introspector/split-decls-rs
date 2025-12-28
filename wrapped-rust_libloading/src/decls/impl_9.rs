macro_rules! deps {
    () => {
        Error!();
        Sealed!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Sealed for String { # [cfg (windows)] fn windows_filename < R > (self , function : impl FnOnce (* const u16) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . windows_filename (function) } # [cfg (unix)] fn posix_filename < R > (mut self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { if crate :: util :: check_null_bytes (self . as_bytes ()) ? { function (self . as_ptr () . cast ()) } else { self . push ('\0') ; function (self . as_ptr () . cast ()) } } }
    };
}

impl_9!()