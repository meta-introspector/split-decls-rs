macro_rules! ZERO_MODULE {
    () => {
        const ZERO_MODULE : ffi :: sqlite3_module = unsafe { ModuleZeroHack { bytes : [0_u8 ; size_of :: < ffi :: sqlite3_module > ()] , } . module } ;
    };
}

ZERO_MODULE!()