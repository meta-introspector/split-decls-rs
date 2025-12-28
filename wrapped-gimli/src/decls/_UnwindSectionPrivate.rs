macro_rules! deps {
    () => {
        Reader!();
        Vendor!();
        Format!();
        CieOffsetEncoding!();
    };
}

macro_rules! _UnwindSectionPrivate {
    () => {
        deps!();
        # [doc = " This trait completely encapsulates everything that is different between"] # [doc = " `.eh_frame` and `.debug_frame`, as well as all the bits that can change"] # [doc = " between DWARF versions."] # [doc (hidden)] pub trait _UnwindSectionPrivate < R : Reader > { # [doc = " Get the underlying section data."] fn section (& self) -> & R ; # [doc = " Returns true if the section allows a zero terminator."] fn has_zero_terminator () -> bool ; # [doc = " Return true if the given offset if the CIE sentinel, false otherwise."] fn is_cie (format : Format , id : u64) -> bool ; # [doc = " Return the CIE offset/ID encoding used by this unwind section with the"] # [doc = " given DWARF format."] fn cie_offset_encoding (format : Format) -> CieOffsetEncoding ; # [doc = " For `.eh_frame`, CIE offsets are relative to the current position. For"] # [doc = " `.debug_frame`, they are relative to the start of the section. We always"] # [doc = " internally store them relative to the section, so we handle translating"] # [doc = " `.eh_frame`'s relative offsets in this method. If the offset calculation"] # [doc = " underflows, return `None`."] fn resolve_cie_offset (& self , base : R :: Offset , offset : R :: Offset) -> Option < R :: Offset > ; # [doc = " Does this version of this unwind section encode address and segment"] # [doc = " sizes in its CIEs?"] fn has_address_and_segment_sizes (version : u8) -> bool ; # [doc = " The address size to use if `has_address_and_segment_sizes` returns false."] fn address_size (& self) -> u8 ; # [doc = " The vendor extensions to use."] fn vendor (& self) -> Vendor ; }
    };
}

_UnwindSectionPrivate!();