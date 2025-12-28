macro_rules! deps {
    () => {
        InlinedFunctionAddress!();
        InlinedFunction!();
        UnitRef!();
        Context!();
        DebugFile!();
    };
}

macro_rules! InlinedState {
    () => {
        deps!();
        struct InlinedState < 'a , R : gimli :: Reader > { entries : gimli :: EntriesRaw < 'a , 'a , R > , functions : Vec < InlinedFunction < R > > , addresses : Vec < InlinedFunctionAddress > , file : DebugFile , unit : gimli :: UnitRef < 'a , R > , ctx : & 'a Context < R > , }
    };
}

InlinedState!()