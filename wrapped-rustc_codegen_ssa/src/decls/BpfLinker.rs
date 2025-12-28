macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! BpfLinker {
    () => {
        deps!();
        struct BpfLinker < 'a > { cmd : Command , sess : & 'a Session , }
    };
}

BpfLinker!()