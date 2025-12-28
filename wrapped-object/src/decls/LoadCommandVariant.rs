macro_rules! deps {
    () => {
        SegmentCommand32!();
        NoteCommand!();
        SubFrameworkCommand!();
        ThreadCommand!();
        RoutinesCommand64!();
        DyldInfoCommand!();
        SymtabCommand!();
        DylinkerCommand!();
        UuidCommand!();
        PreboundDylibCommand!();
        PrebindCksumCommand!();
        FilesetEntryCommand!();
        SourceVersionCommand!();
        BuildVersionCommand!();
        EncryptionInfoCommand32!();
        TwolevelHintsCommand!();
        RoutinesCommand32!();
        Endian!();
        EncryptionInfoCommand64!();
        EntryPointCommand!();
        Dylib!();
        DysymtabCommand!();
        SubLibraryCommand!();
        SegmentCommand64!();
        LinkerOptionCommand!();
        Note!();
        DylibCommand!();
        VersionMinCommand!();
        SubUmbrellaCommand!();
        SubClientCommand!();
        LinkeditDataCommand!();
        RpathCommand!();
    };
}

macro_rules! LoadCommandVariant {
    () => {
        deps!();
        # [doc = " A [`macho::LoadCommand`] that has been interpreted according to its `cmd` field."] # [derive (Debug , Clone , Copy)] # [non_exhaustive] pub enum LoadCommandVariant < 'data , E : Endian > { # [doc = " `LC_SEGMENT`"] Segment32 (& 'data macho :: SegmentCommand32 < E > , & 'data [u8]) , # [doc = " `LC_SYMTAB`"] Symtab (& 'data macho :: SymtabCommand < E >) , # [doc = " `LC_THREAD` or `LC_UNIXTHREAD`"] Thread (& 'data macho :: ThreadCommand < E > , & 'data [u8]) , # [doc = " `LC_DYSYMTAB`"] Dysymtab (& 'data macho :: DysymtabCommand < E >) , # [doc = " `LC_LOAD_DYLIB`, `LC_LOAD_WEAK_DYLIB`, `LC_REEXPORT_DYLIB`,"] # [doc = " `LC_LAZY_LOAD_DYLIB`, or `LC_LOAD_UPWARD_DYLIB`"] Dylib (& 'data macho :: DylibCommand < E >) , # [doc = " `LC_ID_DYLIB`"] IdDylib (& 'data macho :: DylibCommand < E >) , # [doc = " `LC_LOAD_DYLINKER`"] LoadDylinker (& 'data macho :: DylinkerCommand < E >) , # [doc = " `LC_ID_DYLINKER`"] IdDylinker (& 'data macho :: DylinkerCommand < E >) , # [doc = " `LC_PREBOUND_DYLIB`"] PreboundDylib (& 'data macho :: PreboundDylibCommand < E >) , # [doc = " `LC_ROUTINES`"] Routines32 (& 'data macho :: RoutinesCommand32 < E >) , # [doc = " `LC_SUB_FRAMEWORK`"] SubFramework (& 'data macho :: SubFrameworkCommand < E >) , # [doc = " `LC_SUB_UMBRELLA`"] SubUmbrella (& 'data macho :: SubUmbrellaCommand < E >) , # [doc = " `LC_SUB_CLIENT`"] SubClient (& 'data macho :: SubClientCommand < E >) , # [doc = " `LC_SUB_LIBRARY`"] SubLibrary (& 'data macho :: SubLibraryCommand < E >) , # [doc = " `LC_TWOLEVEL_HINTS`"] TwolevelHints (& 'data macho :: TwolevelHintsCommand < E >) , # [doc = " `LC_PREBIND_CKSUM`"] PrebindCksum (& 'data macho :: PrebindCksumCommand < E >) , # [doc = " `LC_SEGMENT_64`"] Segment64 (& 'data macho :: SegmentCommand64 < E > , & 'data [u8]) , # [doc = " `LC_ROUTINES_64`"] Routines64 (& 'data macho :: RoutinesCommand64 < E >) , # [doc = " `LC_UUID`"] Uuid (& 'data macho :: UuidCommand < E >) , # [doc = " `LC_RPATH`"] Rpath (& 'data macho :: RpathCommand < E >) , # [doc = " `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,"] # [doc = " `LC_DATA_IN_CODE`, `LC_DYLIB_CODE_SIGN_DRS`, `LC_LINKER_OPTIMIZATION_HINT`,"] # [doc = " `LC_DYLD_EXPORTS_TRIE`, or `LC_DYLD_CHAINED_FIXUPS`."] LinkeditData (& 'data macho :: LinkeditDataCommand < E >) , # [doc = " `LC_ENCRYPTION_INFO`"] EncryptionInfo32 (& 'data macho :: EncryptionInfoCommand32 < E >) , # [doc = " `LC_DYLD_INFO` or `LC_DYLD_INFO_ONLY`"] DyldInfo (& 'data macho :: DyldInfoCommand < E >) , # [doc = " `LC_VERSION_MIN_MACOSX`, `LC_VERSION_MIN_IPHONEOS`, `LC_VERSION_MIN_WATCHOS`,"] # [doc = " or `LC_VERSION_MIN_TVOS`"] VersionMin (& 'data macho :: VersionMinCommand < E >) , # [doc = " `LC_DYLD_ENVIRONMENT`"] DyldEnvironment (& 'data macho :: DylinkerCommand < E >) , # [doc = " `LC_MAIN`"] EntryPoint (& 'data macho :: EntryPointCommand < E >) , # [doc = " `LC_SOURCE_VERSION`"] SourceVersion (& 'data macho :: SourceVersionCommand < E >) , # [doc = " `LC_ENCRYPTION_INFO_64`"] EncryptionInfo64 (& 'data macho :: EncryptionInfoCommand64 < E >) , # [doc = " `LC_LINKER_OPTION`"] LinkerOption (& 'data macho :: LinkerOptionCommand < E >) , # [doc = " `LC_NOTE`"] Note (& 'data macho :: NoteCommand < E >) , # [doc = " `LC_BUILD_VERSION`"] BuildVersion (& 'data macho :: BuildVersionCommand < E >) , # [doc = " `LC_FILESET_ENTRY`"] FilesetEntry (& 'data macho :: FilesetEntryCommand < E >) , # [doc = " An unrecognized or obsolete load command."] Other , }
    };
}

LoadCommandVariant!()