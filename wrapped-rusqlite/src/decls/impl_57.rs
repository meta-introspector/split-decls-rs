macro_rules! deps {
    () => {
        Result!();
        InnerConnection!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl InnerConnection { # [inline] fn busy_timeout (& mut self , timeout : c_int) -> Result < () > { let r = unsafe { ffi :: sqlite3_busy_timeout (self . db , timeout) } ; self . decode_result (r) } }
    };
}

impl_57!()