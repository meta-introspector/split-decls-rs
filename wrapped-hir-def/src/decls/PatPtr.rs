macro_rules! deps {
    () => {
        Pat!();
    };
}

macro_rules! PatPtr {
    () => {
        deps!();
        pub type PatPtr = AstPtr < ast :: Pat > ;
    };
}

PatPtr!();