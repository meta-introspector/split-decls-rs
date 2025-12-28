macro_rules! deps {
    () => {
        DebugLine!();
        DebugLocLists!();
        DebugStr!();
        DebugRngLists!();
        DebugStrOffsets!();
        DebugInfo!();
        UnitIndex!();
        Reader!();
        DebugTypes!();
        DebugAbbrev!();
        DebugLoc!();
    };
}

macro_rules! DwarfPackage {
    () => {
        deps!();
        # [doc = " The sections from a `.dwp` file, with parsed indices."] # [derive (Debug)] pub struct DwarfPackage < R : Reader > { # [doc = " The compilation unit index in the `.debug_cu_index` section."] pub cu_index : UnitIndex < R > , # [doc = " The type unit index in the `.debug_tu_index` section."] pub tu_index : UnitIndex < R > , # [doc = " The `.debug_abbrev.dwo` section."] pub debug_abbrev : DebugAbbrev < R > , # [doc = " The `.debug_info.dwo` section."] pub debug_info : DebugInfo < R > , # [doc = " The `.debug_line.dwo` section."] pub debug_line : DebugLine < R > , # [doc = " The `.debug_str.dwo` section."] pub debug_str : DebugStr < R > , # [doc = " The `.debug_str_offsets.dwo` section."] pub debug_str_offsets : DebugStrOffsets < R > , # [doc = " The `.debug_loc.dwo` section."] # [doc = ""] # [doc = " Only present when using GNU split-dwarf extension to DWARF 4."] pub debug_loc : DebugLoc < R > , # [doc = " The `.debug_loclists.dwo` section."] pub debug_loclists : DebugLocLists < R > , # [doc = " The `.debug_rnglists.dwo` section."] pub debug_rnglists : DebugRngLists < R > , # [doc = " The `.debug_types.dwo` section."] # [doc = ""] # [doc = " Only present when using GNU split-dwarf extension to DWARF 4."] pub debug_types : DebugTypes < R > , # [doc = " An empty section."] # [doc = ""] # [doc = " Used when creating `Dwarf<R>`."] pub empty : R , }
    };
}

DwarfPackage!();