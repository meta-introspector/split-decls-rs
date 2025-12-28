macro_rules! deps {
    () => {
        RawAutoExtension!();
    };
}

macro_rules! cancel_auto_extension {
    () => {
        deps!();
        # [doc = " Unregister the initialization routine"] pub fn cancel_auto_extension (ax : RawAutoExtension) -> bool { unsafe { ffi :: sqlite3_cancel_auto_extension (Some (ax)) == 1 } }
    };
}

cancel_auto_extension!()