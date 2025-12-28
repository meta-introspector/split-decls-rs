macro_rules! reset_auto_extension {
    () => {
        # [doc = " Disable all automatic extensions previously registered"] pub fn reset_auto_extension () { unsafe { ffi :: sqlite3_reset_auto_extension () } }
    };
}

reset_auto_extension!();