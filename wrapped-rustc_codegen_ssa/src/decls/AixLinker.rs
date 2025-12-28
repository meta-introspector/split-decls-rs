macro_rules! deps {
    () => {
        Command!();
        Linker!();
    };
}

macro_rules! AixLinker {
    () => {
        deps!();
        # [doc = " Linker for AIX."] struct AixLinker < 'a > { cmd : Command , sess : & 'a Session , hinted_static : Option < bool > , }
    };
}

AixLinker!();