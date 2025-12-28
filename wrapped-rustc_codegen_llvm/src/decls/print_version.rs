macro_rules! print_version {
    () => {
        pub (crate) fn print_version () { let (major , minor , patch) = get_version () ; println ! ("LLVM version: {major}.{minor}.{patch}") ; }
    };
}

print_version!();