macro_rules! deps {
    () => {
        Printer!();
        NoAnn!();
        State!();
    };
}

macro_rules! print_crate_as_interface {
    () => {
        deps!();
        pub fn print_crate_as_interface (krate : & ast :: Crate , edition : Edition , g : & AttrIdGenerator ,) -> String { let mut s = State { s : pp :: Printer :: new () , comments : None , ann : & NoAnn , is_sdylib_interface : true } ; print_crate_inner (& mut s , krate , false , edition , g) ; s . s . eof () }
    };
}

print_crate_as_interface!()