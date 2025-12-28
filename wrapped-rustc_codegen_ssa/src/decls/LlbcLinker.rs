macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! LlbcLinker {
    () => {
        deps!();
        # [doc = " The `self-contained` LLVM bitcode linker"] struct LlbcLinker < 'a > { cmd : Command , sess : & 'a Session , }
    };
}

LlbcLinker!()