macro_rules! deps {
    () => {
        MsvcLinker!();
        EmLinker!();
        L4Bender!();
        PtxLinker!();
        Linker!();
        GccLinker!();
        BpfLinker!();
        WasmLd!();
        AixLinker!();
        LlbcLinker!();
    };
}

macro_rules! macro_111 {
    () => {
        deps!();
        generate_arg_methods ! { GccLinker <'_ > MsvcLinker <'_ > EmLinker <'_ > WasmLd <'_ > L4Bender <'_ > AixLinker <'_ > LlbcLinker <'_ > PtxLinker <'_ > BpfLinker <'_ > dyn Linker + '_ }
    };
}

macro_111!();