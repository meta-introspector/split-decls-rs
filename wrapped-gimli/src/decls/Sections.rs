macro_rules! deps {
    () => {
        DebugRngLists!();
        DebugStr!();
        DebugFrame!();
        DebugLoc!();
        DebugLineStr!();
        EhFrame!();
        DebugLine!();
        DebugLocLists!();
        DebugInfoFixup!();
        DebugRanges!();
        DebugAbbrev!();
        DebugInfo!();
        Writer!();
    };
}

macro_rules! Sections {
    () => {
        deps!();
        # [doc = " All of the writable DWARF sections."] # [derive (Debug , Default)] pub struct Sections < W : Writer > { # [doc = " The `.debug_abbrev` section."] pub debug_abbrev : DebugAbbrev < W > , # [doc = " The `.debug_info` section."] pub debug_info : DebugInfo < W > , # [doc = " The `.debug_line` section."] pub debug_line : DebugLine < W > , # [doc = " The `.debug_line_str` section."] pub debug_line_str : DebugLineStr < W > , # [doc = " The `.debug_ranges` section."] pub debug_ranges : DebugRanges < W > , # [doc = " The `.debug_rnglists` section."] pub debug_rnglists : DebugRngLists < W > , # [doc = " The `.debug_loc` section."] pub debug_loc : DebugLoc < W > , # [doc = " The `.debug_loclists` section."] pub debug_loclists : DebugLocLists < W > , # [doc = " The `.debug_str` section."] pub debug_str : DebugStr < W > , # [doc = " The `.debug_frame` section."] pub debug_frame : DebugFrame < W > , # [doc = " The `.eh_frame` section."] pub eh_frame : EhFrame < W > , # [doc = " Unresolved references in the `.debug_info` section."] pub (crate) debug_info_fixups : Vec < DebugInfoFixup > , # [doc = " Unresolved references in the `.debug_loc` section."] pub (crate) debug_loc_fixups : Vec < DebugInfoFixup > , # [doc = " Unresolved references in the `.debug_loclists` section."] pub (crate) debug_loclists_fixups : Vec < DebugInfoFixup > , }
    };
}

Sections!()