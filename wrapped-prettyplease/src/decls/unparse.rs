macro_rules! deps {
    () => {
        Printer!();
    };
}

macro_rules! unparse {
    () => {
        deps!();
        pub fn unparse (file : & File) -> String { let mut p = Printer :: new () ; p . file (file) ; p . eof () }
    };
}

unparse!();