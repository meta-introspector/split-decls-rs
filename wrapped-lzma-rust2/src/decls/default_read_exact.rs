macro_rules! deps {
    () => {
        Read!();
        Result!();
        Error!();
    };
}

macro_rules! default_read_exact {
    () => {
        deps!();
        fn default_read_exact < R : Read + ? Sized > (this : & mut R , mut buf : & mut [u8]) -> crate :: Result < () > { while ! buf . is_empty () { match this . read (buf) { Ok (0) => break , Ok (n) => { buf = & mut buf [n ..] ; } Err (Error :: Interrupted) => { } Err (e) => return Err (e) , } } if ! buf . is_empty () { Err (Error :: Eof) } else { Ok (()) } }
    };
}

default_read_exact!()