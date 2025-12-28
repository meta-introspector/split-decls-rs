macro_rules! deps {
    () => {
        DebugTypesOffset!();
        ReaderOffset!();
        SectionId!();
        DebugTypes!();
        UnitHeader!();
        Reader!();
        UnitOffset!();
        UnitSectionOffset!();
    };
}

macro_rules! impl_604 {
    () => {
        deps!();
        impl < T : ReaderOffset > DebugTypesOffset < T > { # [doc = " Convert a `.debug_types` offset to be an offset within the section containing the"] # [doc = " given unit."] # [doc = ""] # [doc = " Returns `None` if the unit is not within the `.debug_types` section."] pub fn to_unit_section_offset < R > (& self , unit : & UnitHeader < R >) -> Option < UnitSectionOffset < T > > where R : Reader < Offset = T > , { if unit . section != SectionId :: DebugTypes { return None ; } Some (UnitSectionOffset (self . 0)) } # [doc = " Convert an offset to be relative to the start of the given unit,"] # [doc = " instead of relative to the start of the `.debug_types` section."] # [doc = ""] # [doc = " Returns `None` if the offset is not in bounds for the unit's entries."] pub fn to_unit_offset < R > (& self , unit : & UnitHeader < R >) -> Option < UnitOffset < T > > where R : Reader < Offset = T > , { self . to_unit_section_offset (unit) ? . to_unit_offset (unit) } }
    };
}

impl_604!();