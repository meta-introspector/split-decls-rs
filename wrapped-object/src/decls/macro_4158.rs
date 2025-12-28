macro_rules! deps {
    () => {
        Sym64!();
        Sym32!();
        Rela64!();
        SectionHeader64!();
        Versym!();
        NoteHeader32!();
        Rel64!();
        Rela32!();
        Syminfo64!();
        Dyn32!();
        FileHeader64!();
        FileHeader32!();
        CompressionHeader64!();
        Verneed!();
        Verdaux!();
        Relr64!();
        Vernaux!();
        Verdef!();
        ProgramHeader64!();
        GnuHashHeader!();
        Relr32!();
        Rel32!();
        Dyn64!();
        SectionHeader32!();
        CompressionHeader32!();
        Syminfo32!();
        ProgramHeader32!();
        NoteHeader64!();
        HashHeader!();
    };
}

macro_rules! macro_4158 {
    () => {
        deps!();
        unsafe_impl_endian_pod ! (FileHeader32 , FileHeader64 , SectionHeader32 , SectionHeader64 , CompressionHeader32 , CompressionHeader64 , Sym32 , Sym64 , Syminfo32 , Syminfo64 , Rel32 , Rel64 , Rela32 , Rela64 , Relr32 , Relr64 , ProgramHeader32 , ProgramHeader64 , Dyn32 , Dyn64 , Versym , Verdef , Verdaux , Verneed , Vernaux , NoteHeader32 , NoteHeader64 , HashHeader , GnuHashHeader ,) ;
    };
}

macro_4158!()