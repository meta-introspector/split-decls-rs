macro_rules! deps {
    () => {
        DebugLocLists!();
        IndexSectionId!();
        DebugInfo!();
        DebugMacro!();
        DebugTypes!();
        DebugLine!();
        DebugMacinfo!();
        DebugLoc!();
        SectionId!();
        DebugRngLists!();
        DebugStrOffsets!();
        DebugAbbrev!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        impl IndexSectionId { # [doc = " Returns the corresponding `SectionId`."] pub fn section_id (self) -> SectionId { match self { IndexSectionId :: DebugAbbrev => SectionId :: DebugAbbrev , IndexSectionId :: DebugInfo => SectionId :: DebugInfo , IndexSectionId :: DebugLine => SectionId :: DebugLine , IndexSectionId :: DebugLoc => SectionId :: DebugLoc , IndexSectionId :: DebugLocLists => SectionId :: DebugLocLists , IndexSectionId :: DebugMacro => SectionId :: DebugMacro , IndexSectionId :: DebugMacinfo => SectionId :: DebugMacinfo , IndexSectionId :: DebugRngLists => SectionId :: DebugRngLists , IndexSectionId :: DebugStrOffsets => SectionId :: DebugStrOffsets , IndexSectionId :: DebugTypes => SectionId :: DebugTypes , } } # [doc = " Returns the ELF section name for this kind, when found in a .dwo or .dwp file."] pub fn dwo_name (self) -> & 'static str { self . section_id () . dwo_name () . unwrap () } }
    };
}

impl_396!()