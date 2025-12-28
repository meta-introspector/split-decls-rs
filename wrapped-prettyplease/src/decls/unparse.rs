macro_rules! unparse {
    () => {
        pub fn unparse (file : & File) -> String { let mut p = Printer :: new () ; p . file (file) ; p . eof () }
    };
}

unparse!()