macro_rules! deps {
    () => {
        DefPathTable!();
    };
}

macro_rules! Definitions {
    () => {
        deps!();
        # [doc = " The definition table containing node definitions."] # [doc = " It holds the `DefPathTable` for `LocalDefId`s/`DefPath`s."] # [doc = " It also stores mappings to convert `LocalDefId`s to/from `HirId`s."] # [derive (Debug)] pub struct Definitions { table : DefPathTable , }
    };
}

Definitions!();