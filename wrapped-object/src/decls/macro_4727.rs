macro_rules! deps {
    () => {
        DylibTableOfContents!();
        SegmentCommand32!();
        Section64!();
        FilesetEntryCommand!();
        DyldCacheMappingInfo!();
        UuidCommand!();
        SubUmbrellaCommand!();
        SourceVersionCommand!();
        SymtabCommand!();
        VersionMinCommand!();
        DylibModule64!();
        Nlist32!();
        SegmentCommand64!();
        DyldSubCacheEntryV2!();
        Section32!();
        PrebindCksumCommand!();
        DyldSubCacheEntryV1!();
        DyldCacheSlideInfo5!();
        LoadCommand!();
        FvmfileCommand!();
        DyldCacheImageInfo!();
        RoutinesCommand32!();
        DylibModule32!();
        EntryPointCommand!();
        Relocation!();
        DataInCodeEntry!();
        DylibCommand!();
        MachHeader64!();
        NoteCommand!();
        TwolevelHintsCommand!();
        MachHeader32!();
        Nlist64!();
        RpathCommand!();
        LinkerOptionCommand!();
        DyldCacheSlideInfo3!();
        TwolevelHint!();
        SymsegCommand!();
        DylibReference!();
        DyldInfoCommand!();
        SubLibraryCommand!();
        DylinkerCommand!();
        DyldCacheHeader!();
        IdentCommand!();
        DyldCacheSlideInfo2!();
        SubFrameworkCommand!();
        RoutinesCommand64!();
        BuildToolVersion!();
        Fvmlib!();
        EncryptionInfoCommand32!();
        PreboundDylibCommand!();
        FvmlibCommand!();
        LinkeditDataCommand!();
        EncryptionInfoCommand64!();
        DyldCacheMappingAndSlideInfo!();
        Dylib!();
        LcStr!();
        DysymtabCommand!();
        BuildVersionCommand!();
        ThreadCommand!();
        SubClientCommand!();
    };
}

macro_rules! macro_4727 {
    () => {
        deps!();
        unsafe_impl_endian_pod ! (DyldCacheHeader , DyldCacheMappingInfo , DyldCacheMappingAndSlideInfo , DyldCacheImageInfo , DyldCacheSlideInfo2 , DyldCacheSlideInfo3 , DyldCacheSlideInfo5 , DyldSubCacheEntryV1 , DyldSubCacheEntryV2 , MachHeader32 , MachHeader64 , LoadCommand , LcStr , SegmentCommand32 , SegmentCommand64 , Section32 , Section64 , Fvmlib , FvmlibCommand , Dylib , DylibCommand , SubFrameworkCommand , SubClientCommand , SubUmbrellaCommand , SubLibraryCommand , PreboundDylibCommand , DylinkerCommand , ThreadCommand , RoutinesCommand32 , RoutinesCommand64 , SymtabCommand , DysymtabCommand , DylibTableOfContents , DylibModule32 , DylibModule64 , DylibReference , TwolevelHintsCommand , TwolevelHint , PrebindCksumCommand , UuidCommand , RpathCommand , LinkeditDataCommand , FilesetEntryCommand , EncryptionInfoCommand32 , EncryptionInfoCommand64 , VersionMinCommand , BuildVersionCommand , BuildToolVersion , DyldInfoCommand , LinkerOptionCommand , SymsegCommand , IdentCommand , FvmfileCommand , EntryPointCommand , SourceVersionCommand , DataInCodeEntry , NoteCommand , Nlist32 , Nlist64 , Relocation ,) ;
    };
}

macro_4727!()