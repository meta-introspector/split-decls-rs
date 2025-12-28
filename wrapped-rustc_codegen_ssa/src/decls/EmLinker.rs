macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! EmLinker {
    () => {
        deps!();
        struct EmLinker < 'a > { cmd : Command , sess : & 'a Session , }
    };
}

EmLinker!()