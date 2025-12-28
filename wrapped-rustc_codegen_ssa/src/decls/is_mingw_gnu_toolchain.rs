macro_rules! is_mingw_gnu_toolchain {
    () => {
        pub fn is_mingw_gnu_toolchain (target : & Target) -> bool { target . vendor == "pc" && target . os == "windows" && target . env == "gnu" && target . abi . is_empty () }
    };
}

is_mingw_gnu_toolchain!()