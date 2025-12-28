macro_rules! CR {
    () => {
        pub const CR : & 'static [(char , char)] = & [('\r' , '\r')] ;
    };
}

CR!();