macro_rules! deps {
    () => {
        Sealed!();
        Error!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Sealed for & str { # [cfg (windows)] fn windows_filename < R > (self , function : impl FnOnce (* const u16) -> Result < R , Error > ,) -> Result < R , Error > { let utf16 : alloc :: vec :: Vec < u16 > = if crate :: util :: check_null_bytes (self . as_bytes ()) ? { self . encode_utf16 () . collect () } else { self . encode_utf16 () . chain (Some (0)) . collect () } ; function (utf16 . as_ptr ()) } # [cfg (unix)] fn posix_filename < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { if crate :: util :: check_null_bytes (self . as_bytes ()) ? { function (self . as_ptr () . cast ()) } else { let buffer = crate :: util :: copy_and_push (self . as_bytes () , 0) ; function (buffer . as_ptr () . cast ()) } } }
    };
}

impl_5!();