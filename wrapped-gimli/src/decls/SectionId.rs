macro_rules! deps {
    () => {
        DebugLine!();
        DebugInfo!();
        DebugLoc!();
        DebugLocLists!();
        EhFrame!();
        EhFrameHdr!();
        DebugAddr!();
        DebugCuIndex!();
        DebugRanges!();
        DebugRngLists!();
        DebugTypes!();
        DebugPubNames!();
        DebugTuIndex!();
        DebugLineStr!();
        DebugFrame!();
        DebugMacinfo!();
        DebugMacro!();
        DebugPubTypes!();
        DebugAranges!();
        DebugStrOffsets!();
        DebugAbbrev!();
        DebugStr!();
    };
}

macro_rules! SectionId {
    () => {
        deps!();
        # [doc = " An identifier for a DWARF section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] pub enum SectionId { # [doc = " The `.debug_abbrev` section."] DebugAbbrev , # [doc = " The `.debug_addr` section."] DebugAddr , # [doc = " The `.debug_aranges` section."] DebugAranges , # [doc = " The `.debug_cu_index` section."] DebugCuIndex , # [doc = " The `.debug_frame` section."] DebugFrame , # [doc = " The `.eh_frame` section."] EhFrame , # [doc = " The `.eh_frame_hdr` section."] EhFrameHdr , # [doc = " The `.debug_info` section."] DebugInfo , # [doc = " The `.debug_line` section."] DebugLine , # [doc = " The `.debug_line_str` section."] DebugLineStr , # [doc = " The `.debug_loc` section."] DebugLoc , # [doc = " The `.debug_loclists` section."] DebugLocLists , # [doc = " The `.debug_macinfo` section."] DebugMacinfo , # [doc = " The `.debug_macro` section."] DebugMacro , # [doc = " The `.debug_pubnames` section."] DebugPubNames , # [doc = " The `.debug_pubtypes` section."] DebugPubTypes , # [doc = " The `.debug_ranges` section."] DebugRanges , # [doc = " The `.debug_rnglists` section."] DebugRngLists , # [doc = " The `.debug_str` section."] DebugStr , # [doc = " The `.debug_str_offsets` section."] DebugStrOffsets , # [doc = " The `.debug_tu_index` section."] DebugTuIndex , # [doc = " The `.debug_types` section."] DebugTypes , }
    };
}

SectionId!()