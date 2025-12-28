macro_rules! deps {
    () => {
        Demangle!();
        TryDemangleError!();
    };
}

macro_rules! try_demangle {
    () => {
        deps!();
        # [doc = " The same as `demangle`, except return an `Err` if the string does not appear"] # [doc = " to be a Rust symbol, rather than \"demangling\" the given string as a no-op."] # [doc = ""] # [doc = " ```"] # [doc = " extern crate rustc_demangle;"] # [doc = ""] # [doc = " let not_a_rust_symbol = \"la la la\";"] # [doc = ""] # [doc = " // The `try_demangle` function will reject strings which are not Rust symbols."] # [doc = " assert!(rustc_demangle::try_demangle(not_a_rust_symbol).is_err());"] # [doc = ""] # [doc = " // While `demangle` will just pass the non-symbol through as a no-op."] # [doc = " assert_eq!(rustc_demangle::demangle(not_a_rust_symbol).as_str(), not_a_rust_symbol);"] # [doc = " ```"] pub fn try_demangle (s : & str) -> Result < Demangle , TryDemangleError > { let sym = demangle (s) ; if sym . style . is_some () { Ok (sym) } else { Err (TryDemangleError { _priv : () }) } }
    };
}

try_demangle!()