macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! MsvcLinker {
    () => {
        deps!();
        struct MsvcLinker < 'a > { cmd : Command , sess : & 'a Session , }
    };
}

MsvcLinker!()