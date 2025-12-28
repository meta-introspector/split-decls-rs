macro_rules! deps {
    () => {
        DwarfAux64!();
        SectionHeader64!();
        FileHeader64!();
        CsectAux64!();
        StatAux!();
        FileAux32!();
        Rel64!();
        Symbol32!();
        FileHeader32!();
        FileAux64!();
        FunAux64!();
        AuxHeader64!();
        Symbol64!();
        ExpAux!();
        SectionHeader32!();
        BlockAux32!();
        DwarfAux32!();
        Rel32!();
        CsectAux32!();
        BlockAux64!();
        AuxHeader32!();
        FunAux32!();
        SymbolBytes!();
    };
}

macro_rules! macro_5694 {
    () => {
        deps!();
        unsafe_impl_pod ! (FileHeader32 , FileHeader64 , AuxHeader32 , AuxHeader64 , SectionHeader32 , SectionHeader64 , SymbolBytes , Symbol32 , Symbol64 , FileAux32 , FileAux64 , CsectAux32 , CsectAux64 , FunAux32 , FunAux64 , ExpAux , BlockAux32 , BlockAux64 , StatAux , DwarfAux32 , DwarfAux64 , Rel32 , Rel64 ,) ;
    };
}

macro_5694!()