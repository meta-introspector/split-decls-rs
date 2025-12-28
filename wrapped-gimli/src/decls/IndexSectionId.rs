macro_rules! deps {
    () => {
        DebugMacro!();
        DebugInfo!();
        DebugLocLists!();
        DebugRngLists!();
        DebugStrOffsets!();
        DebugAbbrev!();
        DebugLoc!();
        DebugMacinfo!();
        Section!();
        DebugLine!();
        DebugTypes!();
    };
}

macro_rules! IndexSectionId {
    () => {
        deps!();
        # [doc = " Section kinds which are permitted in a `.dwp` index."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum IndexSectionId { # [doc = " The `.debug_abbrev.dwo` section."] DebugAbbrev , # [doc = " The `.debug_info.dwo` section."] DebugInfo , # [doc = " The `.debug_line.dwo` section."] DebugLine , # [doc = " The `.debug_loc.dwo` section."] DebugLoc , # [doc = " The `.debug_loclists.dwo` section."] DebugLocLists , # [doc = " The `.debug_macinfo.dwo` section."] DebugMacinfo , # [doc = " The `.debug_macro.dwo` section."] DebugMacro , # [doc = " The `.debug_rnglists.dwo` section."] DebugRngLists , # [doc = " The `.debug_str_offsets.dwo` section."] DebugStrOffsets , # [doc = " The `.debug_types.dwo` section."] DebugTypes , }
    };
}

IndexSectionId!();