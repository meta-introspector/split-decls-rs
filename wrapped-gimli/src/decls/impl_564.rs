macro_rules! deps {
    () => {
        RangeLists!();
        DebugRanges!();
        DebugRngLists!();
    };
}

macro_rules! impl_564 {
    () => {
        deps!();
        impl < R > RangeLists < R > { # [doc = " Construct a new `RangeLists` instance from the data in the `.debug_ranges` and"] # [doc = " `.debug_rnglists` sections."] pub fn new (debug_ranges : DebugRanges < R > , debug_rnglists : DebugRngLists < R >) -> RangeLists < R > { RangeLists { debug_ranges , debug_rnglists , } } # [doc = " Return the `.debug_ranges` section."] pub fn debug_ranges (& self) -> & DebugRanges < R > { & self . debug_ranges } # [doc = " Replace the `.debug_ranges` section."] # [doc = ""] # [doc = " This is useful for `.dwo` files when using the GNU split-dwarf extension to DWARF 4."] pub fn set_debug_ranges (& mut self , debug_ranges : DebugRanges < R >) { self . debug_ranges = debug_ranges ; } # [doc = " Return the `.debug_rnglists` section."] pub fn debug_rnglists (& self) -> & DebugRngLists < R > { & self . debug_rnglists } }
    };
}

impl_564!();