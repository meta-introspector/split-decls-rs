macro_rules! SupUnit {
    () => {
        pub (crate) struct SupUnit < R : gimli :: Reader > { offset : gimli :: DebugInfoOffset < R :: Offset > , dw_unit : gimli :: Unit < R > , }
    };
}

SupUnit!();