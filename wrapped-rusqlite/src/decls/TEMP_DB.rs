macro_rules! TEMP_DB {
    () => {
        # [doc = " Shorthand for `Temp` database."] pub const TEMP_DB : & CStr = c"temp" ;
    };
}

TEMP_DB!()