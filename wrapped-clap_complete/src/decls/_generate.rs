macro_rules! deps {
    () => {
        Generator!();
    };
}

macro_rules! _generate {
    () => {
        deps!();
        fn _generate < G : Generator > (generator : G , cmd : & mut Command , buf : & mut dyn Write) { cmd . build () ; generator . generate (cmd , buf) ; }
    };
}

_generate!()