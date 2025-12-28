macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! simple_array {
    () => {
        deps!();
        pub fn simple_array (elements : & Punctuated < Expr , Token ! [,] >) -> bool { for expr in elements { if let Expr :: Lit (expr) = expr { match expr . lit { # ! [cfg_attr (all (test , exhaustive) , deny (non_exhaustive_omitted_patterns))] Lit :: Byte (_) | Lit :: Char (_) | Lit :: Int (_) | Lit :: Bool (_) => { } Lit :: Str (_) | Lit :: ByteStr (_) | Lit :: CStr (_) | Lit :: Float (_) | Lit :: Verbatim (_) => { return false ; } _ => return false , } } else { return false ; } } true }
    };
}

simple_array!();