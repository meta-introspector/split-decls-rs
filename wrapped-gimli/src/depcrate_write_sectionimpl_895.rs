// Generated macro for impl_895 (impl)
macro_rules! Depcrate_write_sectionimpl_895 {
() => {
// Module: crate::write::section
// Provides: {"impl_895"}
// Dependencies: {}
impl < W : Writer + Clone > Sections < W > { # [doc = " Create a new `Sections` using clones of the given `section`."] pub fn new (section : W) -> Self { Sections { debug_abbrev : DebugAbbrev (section . clone ()) , debug_info : DebugInfo (section . clone ()) , debug_line : DebugLine (section . clone ()) , debug_line_str : DebugLineStr (section . clone ()) , debug_ranges : DebugRanges (section . clone ()) , debug_rnglists : DebugRngLists (section . clone ()) , debug_loc : DebugLoc (section . clone ()) , debug_loclists : DebugLocLists (section . clone ()) , debug_str : DebugStr (section . clone ()) , debug_frame : DebugFrame (section . clone ()) , eh_frame : EhFrame (section) , debug_info_fixups : Vec :: new () , debug_loc_fixups : Vec :: new () , debug_loclists_fixups : Vec :: new () , } } }
};
}
