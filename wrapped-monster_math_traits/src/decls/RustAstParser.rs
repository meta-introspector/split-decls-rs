macro_rules! deps {
    () => {
        Declaration!();
    };
}

macro_rules! RustAstParser {
    () => {
        deps!();
        # [doc = " A trait for parsing Rust source code and extracting structured declarations."] pub trait RustAstParser { # [doc = " Parses a given string of Rust source code and returns a vector of `Declaration`s."] fn parse_rust_code (& self , code : & str) -> Vec < Declaration > ; }
    };
}

RustAstParser!()