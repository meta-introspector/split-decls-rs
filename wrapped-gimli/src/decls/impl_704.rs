macro_rules! deps {
    () => {
        DebugAbbrev!();
        DebugInfo!();
        Sections!();
        DebugLocLists!();
        EhFrame!();
        DebugLine!();
        DebugLoc!();
        DebugFrame!();
        Writer!();
        DebugRanges!();
        DebugStr!();
        DebugLineStr!();
        DebugRngLists!();
    };
}

macro_rules! impl_704 {
    () => {
        deps!();
        impl < W : Writer + Clone > Sections < W > { # [doc = " Create a new `Sections` using clones of the given `section`."] pub fn new (section : W) -> Self { Sections { debug_abbrev : DebugAbbrev (section . clone ()) , debug_info : DebugInfo (section . clone ()) , debug_line : DebugLine (section . clone ()) , debug_line_str : DebugLineStr (section . clone ()) , debug_ranges : DebugRanges (section . clone ()) , debug_rnglists : DebugRngLists (section . clone ()) , debug_loc : DebugLoc (section . clone ()) , debug_loclists : DebugLocLists (section . clone ()) , debug_str : DebugStr (section . clone ()) , debug_frame : DebugFrame (section . clone ()) , eh_frame : EhFrame (section) , debug_info_fixups : Vec :: new () , debug_loc_fixups : Vec :: new () , debug_loclists_fixups : Vec :: new () , } } }
    };
}

impl_704!()