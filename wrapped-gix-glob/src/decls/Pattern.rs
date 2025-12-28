macro_rules! deps {
    () => {
        Mapping!();
    };
}

macro_rules! Pattern {
    () => {
        deps!();
        # [doc = " A trait to convert bytes into patterns and their associated value."] # [doc = ""] # [doc = " This is used for `gitattributes` which have a value, and `gitignore` which don't."] pub trait Pattern : Clone + PartialEq + Eq + std :: fmt :: Debug + std :: hash :: Hash + Ord + PartialOrd + Default { # [doc = " The value associated with a pattern."] type Value : PartialEq + Eq + std :: fmt :: Debug + std :: hash :: Hash + Ord + PartialOrd + Clone ; # [doc = " Parse all patterns in `bytes` line by line, ignoring lines with errors, and collect them."] fn bytes_to_patterns (& self , bytes : & [u8] , source : & Path) -> Vec < pattern :: Mapping < Self :: Value > > ; }
    };
}

Pattern!()