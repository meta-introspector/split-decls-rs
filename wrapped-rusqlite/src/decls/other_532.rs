macro_rules! other_532 {
    () => {
        union ModuleZeroHack { bytes : [u8 ; size_of :: < ffi :: sqlite3_module > ()] , module : ffi :: sqlite3_module , }
    };
}

other_532!()