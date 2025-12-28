macro_rules! deps {
    () => {
        DyldSubCacheEntryV2!();
        TwolevelHintsCommand!();
        Nlist64!();
        DyldSubCacheEntryV1!();
        BuildToolVersion!();
        EntryPointCommand!();
        DyldCacheSlideInfo5!();
        MachHeader64!();
        Section64!();
        DylibTableOfContents!();
        FvmfileCommand!();
        SubLibraryCommand!();
        DysymtabCommand!();
        SubFrameworkCommand!();
        SourceVersionCommand!();
        LcStr!();
        LinkerOptionCommand!();
        TwolevelHint!();
        PreboundDylibCommand!();
        DylibModule64!();
        FilesetEntryCommand!();
        DataInCodeEntry!();
        DylibModule32!();
        ThreadCommand!();
        DyldInfoCommand!();
        RoutinesCommand32!();
        DyldCacheMappingAndSlideInfo!();
        SegmentCommand32!();
        SymsegCommand!();
        DyldCacheHeader!();
        RpathCommand!();
        SegmentCommand64!();
        IdentCommand!();
        NoteCommand!();
        Section32!();
        DylibCommand!();
        RoutinesCommand64!();
        DyldCacheSlideInfo2!();
        DyldCacheSlideInfo3!();
        Fvmlib!();
        DylibReference!();
        DyldCacheImageInfo!();
        Dylib!();
        SubUmbrellaCommand!();
        VersionMinCommand!();
        Relocation!();
        MachHeader32!();
        SubClientCommand!();
        SymtabCommand!();
        DylinkerCommand!();
        PrebindCksumCommand!();
        EncryptionInfoCommand32!();
        DyldCacheMappingInfo!();
        LinkeditDataCommand!();
        EncryptionInfoCommand64!();
        FvmlibCommand!();
        UuidCommand!();
        Nlist32!();
        LoadCommand!();
        BuildVersionCommand!();
    };
}

macro_rules! macro_4727 {
    () => {
        deps!();
        unsafe_impl_endian_pod ! (DyldCacheHeader , DyldCacheMappingInfo , DyldCacheMappingAndSlideInfo , DyldCacheImageInfo , DyldCacheSlideInfo2 , DyldCacheSlideInfo3 , DyldCacheSlideInfo5 , DyldSubCacheEntryV1 , DyldSubCacheEntryV2 , MachHeader32 , MachHeader64 , LoadCommand , LcStr , SegmentCommand32 , SegmentCommand64 , Section32 , Section64 , Fvmlib , FvmlibCommand , Dylib , DylibCommand , SubFrameworkCommand , SubClientCommand , SubUmbrellaCommand , SubLibraryCommand , PreboundDylibCommand , DylinkerCommand , ThreadCommand , RoutinesCommand32 , RoutinesCommand64 , SymtabCommand , DysymtabCommand , DylibTableOfContents , DylibModule32 , DylibModule64 , DylibReference , TwolevelHintsCommand , TwolevelHint , PrebindCksumCommand , UuidCommand , RpathCommand , LinkeditDataCommand , FilesetEntryCommand , EncryptionInfoCommand32 , EncryptionInfoCommand64 , VersionMinCommand , BuildVersionCommand , BuildToolVersion , DyldInfoCommand , LinkerOptionCommand , SymsegCommand , IdentCommand , FvmfileCommand , EntryPointCommand , SourceVersionCommand , DataInCodeEntry , NoteCommand , Nlist32 , Nlist64 , Relocation ,) ;
    };
}

macro_4727!();