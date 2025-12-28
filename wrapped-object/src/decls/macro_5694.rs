macro_rules! deps {
    () => {
        SectionHeader64!();
        FunAux32!();
        DwarfAux32!();
        ExpAux!();
        AuxHeader64!();
        SectionHeader32!();
        Symbol64!();
        CsectAux32!();
        FileAux32!();
        CsectAux64!();
        StatAux!();
        AuxHeader32!();
        FunAux64!();
        FileHeader64!();
        DwarfAux64!();
        Rel32!();
        Rel64!();
        SymbolBytes!();
        FileHeader32!();
        Symbol32!();
        BlockAux32!();
        FileAux64!();
        BlockAux64!();
    };
}

macro_rules! macro_5694 {
    () => {
        deps!();
        unsafe_impl_pod ! (FileHeader32 , FileHeader64 , AuxHeader32 , AuxHeader64 , SectionHeader32 , SectionHeader64 , SymbolBytes , Symbol32 , Symbol64 , FileAux32 , FileAux64 , CsectAux32 , CsectAux64 , FunAux32 , FunAux64 , ExpAux , BlockAux32 , BlockAux64 , StatAux , DwarfAux32 , DwarfAux64 , Rel32 , Rel64 ,) ;
    };
}

macro_5694!();