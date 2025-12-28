macro_rules! deps {
    () => {
        EmLinker!();
        PtxLinker!();
        Linker!();
        LlbcLinker!();
        AixLinker!();
        BpfLinker!();
        L4Bender!();
        MsvcLinker!();
        WasmLd!();
        GccLinker!();
    };
}

macro_rules! macro_111 {
    () => {
        deps!();
        generate_arg_methods ! { GccLinker <'_ > MsvcLinker <'_ > EmLinker <'_ > WasmLd <'_ > L4Bender <'_ > AixLinker <'_ > LlbcLinker <'_ > PtxLinker <'_ > BpfLinker <'_ > dyn Linker + '_ }
    };
}

macro_111!()