macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! PtxLinker {
    () => {
        deps!();
        # [doc = " Much simplified and explicit CLI for the NVPTX linker. The linker operates"] # [doc = " with bitcode and uses LLVM backend to generate a PTX assembly."] struct PtxLinker < 'a > { cmd : Command , sess : & 'a Session , }
    };
}

PtxLinker!();