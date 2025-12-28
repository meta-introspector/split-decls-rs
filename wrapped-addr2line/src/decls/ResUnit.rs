macro_rules! deps {
    () => {
        LazyFunctions!();
        LazyLines!();
        DwoUnit!();
        LazyResult!();
    };
}

macro_rules! ResUnit {
    () => {
        deps!();
        pub (crate) struct ResUnit < R : gimli :: Reader > { offset : gimli :: DebugInfoOffset < R :: Offset > , dw_unit : gimli :: Unit < R > , pub (crate) lang : Option < gimli :: DwLang > , lines : LazyLines , functions : LazyFunctions < R > , dwo : LazyResult < Option < Box < DwoUnit < R > > > > , }
    };
}

ResUnit!();