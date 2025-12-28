macro_rules! deps {
    () => {
        Syminfo32!();
        ProgramHeader32!();
        Dyn64!();
        NoteHeader32!();
        Versym!();
        Rela32!();
        FileHeader64!();
        Verneed!();
        Rel32!();
        Relr32!();
        Rela64!();
        Dyn32!();
        NoteHeader64!();
        GnuHashHeader!();
        SectionHeader64!();
        Sym32!();
        Rel64!();
        FileHeader32!();
        Verdaux!();
        SectionHeader32!();
        CompressionHeader32!();
        Syminfo64!();
        Verdef!();
        Relr64!();
        HashHeader!();
        CompressionHeader64!();
        ProgramHeader64!();
        Sym64!();
        Vernaux!();
    };
}

macro_rules! macro_4158 {
    () => {
        deps!();
        unsafe_impl_endian_pod ! (FileHeader32 , FileHeader64 , SectionHeader32 , SectionHeader64 , CompressionHeader32 , CompressionHeader64 , Sym32 , Sym64 , Syminfo32 , Syminfo64 , Rel32 , Rel64 , Rela32 , Rela64 , Relr32 , Relr64 , ProgramHeader32 , ProgramHeader64 , Dyn32 , Dyn64 , Versym , Verdef , Verdaux , Verneed , Vernaux , NoteHeader32 , NoteHeader64 , HashHeader , GnuHashHeader ,) ;
    };
}

macro_4158!();