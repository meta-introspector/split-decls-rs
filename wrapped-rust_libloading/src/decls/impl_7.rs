macro_rules! deps {
    () => {
        Error!();
        Sealed!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Sealed for & String { # [cfg (windows)] fn windows_filename < R > (self , function : impl FnOnce (* const u16) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . windows_filename (function) } # [cfg (unix)] fn posix_filename < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . posix_filename (function) } }
    };
}

impl_7!()