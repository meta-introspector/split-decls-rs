macro_rules! deps {
    () => {
        Linker!();
        Command!();
    };
}

macro_rules! L4Bender {
    () => {
        deps!();
        # [doc = " Linker shepherd script for L4Re (Fiasco)"] struct L4Bender < 'a > { cmd : Command , sess : & 'a Session , hinted_static : bool , }
    };
}

L4Bender!()