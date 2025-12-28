macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! provide {
    () => {
        deps!();
        pub fn provide < 'a > (err : & 'a (impl Error + ? Sized) , request : & mut Request < 'a >) { Error :: provide (err , request) ; }
    };
}

provide!();