macro_rules! deps {
    () => {
        DwarfPackage!();
        DebugStrOffsets!();
        DebugLine!();
        Reader!();
        DebugStr!();
        DebugTypes!();
        Result!();
        DebugLoc!();
        DebugRngLists!();
        DebugCuIndex!();
        DebugLocLists!();
        DebugInfo!();
        DebugAbbrev!();
        DebugTuIndex!();
    };
}

macro_rules! DwarfPackageSections {
    () => {
        deps!();
        # [doc = " The sections from a `.dwp` file."] # [doc = ""] # [doc = " This is useful for storing sections when `T` does not implement `Reader`."] # [doc = " It can be used to create a `DwarfPackage` that references the data in `self`."] # [doc = " If `T` does implement `Reader`, then use `DwarfPackage` directly."] # [doc = ""] # [doc = " ## Example Usage"] # [doc = ""] # [doc = " It can be useful to load DWARF sections into owned data structures,"] # [doc = " such as `Vec`. However, we do not implement the `Reader` trait"] # [doc = " for `Vec`, because it would be very inefficient, but this trait"] # [doc = " is required for all of the methods that parse the DWARF data."] # [doc = " So we first load the DWARF sections into `Vec`s, and then use"] # [doc = " `borrow` to create `Reader`s that reference the data."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # fn example() -> Result<(), gimli::Error> {"] # [doc = " # let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };"] # [doc = " // Read the DWARF sections into `Vec`s with whatever object loader you're using."] # [doc = " let dwp_sections: gimli::DwarfPackageSections<Vec<u8>> = gimli::DwarfPackageSections::load(loader)?;"] # [doc = " // Create references to the DWARF sections."] # [doc = " let dwp: gimli::DwarfPackage<_> = dwp_sections.borrow("] # [doc = "     |section| gimli::EndianSlice::new(&section, gimli::LittleEndian),"] # [doc = "     gimli::EndianSlice::new(&[], gimli::LittleEndian),"] # [doc = " )?;"] # [doc = " # unreachable!()"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug , Default)] pub struct DwarfPackageSections < T > { # [doc = " The `.debug_cu_index` section."] pub cu_index : DebugCuIndex < T > , # [doc = " The `.debug_tu_index` section."] pub tu_index : DebugTuIndex < T > , # [doc = " The `.debug_abbrev.dwo` section."] pub debug_abbrev : DebugAbbrev < T > , # [doc = " The `.debug_info.dwo` section."] pub debug_info : DebugInfo < T > , # [doc = " The `.debug_line.dwo` section."] pub debug_line : DebugLine < T > , # [doc = " The `.debug_str.dwo` section."] pub debug_str : DebugStr < T > , # [doc = " The `.debug_str_offsets.dwo` section."] pub debug_str_offsets : DebugStrOffsets < T > , # [doc = " The `.debug_loc.dwo` section."] # [doc = ""] # [doc = " Only present when using GNU split-dwarf extension to DWARF 4."] pub debug_loc : DebugLoc < T > , # [doc = " The `.debug_loclists.dwo` section."] pub debug_loclists : DebugLocLists < T > , # [doc = " The `.debug_rnglists.dwo` section."] pub debug_rnglists : DebugRngLists < T > , # [doc = " The `.debug_types.dwo` section."] # [doc = ""] # [doc = " Only present when using GNU split-dwarf extension to DWARF 4."] pub debug_types : DebugTypes < T > , }
    };
}

DwarfPackageSections!();