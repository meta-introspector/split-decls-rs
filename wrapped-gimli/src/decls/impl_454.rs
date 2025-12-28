macro_rules! deps {
    () => {
        DebugLoc!();
        LocationLists!();
        DebugLocLists!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl < R > LocationLists < R > { # [doc = " Construct a new `LocationLists` instance from the data in the `.debug_loc` and"] # [doc = " `.debug_loclists` sections."] pub fn new (debug_loc : DebugLoc < R > , debug_loclists : DebugLocLists < R >) -> LocationLists < R > { LocationLists { debug_loc , debug_loclists , } } }
    };
}

impl_454!();