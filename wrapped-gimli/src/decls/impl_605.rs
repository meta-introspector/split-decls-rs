macro_rules! deps {
    () => {
        UnitOffset!();
        ReaderOffset!();
        Reader!();
        UnitHeader!();
        SectionId!();
        UnitSectionOffset!();
        DebugInfo!();
        DebugInfoOffset!();
        DebugTypesOffset!();
        DebugTypes!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        impl < T : ReaderOffset > UnitSectionOffset < T > { # [doc = " Convert an offset to be relative to the start of the given unit,"] # [doc = " instead of relative to the start of the section."] # [doc = ""] # [doc = " Returns `None` if the offset is not in bounds for the unit's entries."] pub fn to_unit_offset < R > (& self , unit : & UnitHeader < R >) -> Option < UnitOffset < T > > where R : Reader < Offset = T > , { let offset = UnitOffset (self . 0 . checked_sub (unit . offset () . 0) ?) ; if ! unit . is_in_bounds (offset) { return None ; } Some (offset) } # [doc = " Convert an offset to be relative to the start of the `.debug_info` section,"] # [doc = " instead of relative to the start of the section for the given unit."] # [doc = ""] # [doc = " Returns `None` if the unit is not within the `.debug_info` section."] pub fn to_debug_info_offset < R > (& self , unit : & UnitHeader < R >) -> Option < DebugInfoOffset < T > > where R : Reader < Offset = T > , { if unit . section != SectionId :: DebugInfo { return None ; } Some (DebugInfoOffset (self . 0)) } # [doc = " Convert an offset to be relative to the start of the `.debug_types` section,"] # [doc = " instead of relative to the start of the section for the given unit."] # [doc = ""] # [doc = " Returns `None` if the unit is not within the `.debug_types` section."] pub fn to_debug_types_offset < R > (& self , unit : & UnitHeader < R >) -> Option < DebugTypesOffset < T > > where R : Reader < Offset = T > , { if unit . section != SectionId :: DebugTypes { return None ; } Some (DebugTypesOffset (self . 0)) } }
    };
}

impl_605!()