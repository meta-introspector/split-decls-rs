macro_rules! ffi {
    () => {
        # [doc = " A module full of foreign C functions."] # [automock] pub mod ffi { extern "C" { # [doc = " A foreign \"C\" function."] pub fn ffi_func () ; } }
    };
}

ffi!();