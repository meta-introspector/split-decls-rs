// Generated macro for impl_572 (impl)
macro_rules! Depcrate_read_loclistsimpl_572 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_572"}
// Dependencies: {}
impl < R > LocationLists < R > { # [doc = " Construct a new `LocationLists` instance from the data in the `.debug_loc` and"] # [doc = " `.debug_loclists` sections."] pub fn new (debug_loc : DebugLoc < R > , debug_loclists : DebugLocLists < R >) -> LocationLists < R > { LocationLists { debug_loc , debug_loclists , } } }
};
}
