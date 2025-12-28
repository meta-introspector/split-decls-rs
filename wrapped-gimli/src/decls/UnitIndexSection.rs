macro_rules! deps {
    () => {
        IndexSectionId!();
    };
}

macro_rules! UnitIndexSection {
    () => {
        deps!();
        # [doc = " Information about a unit's contribution to a section in a `.dwp` file."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct UnitIndexSection { # [doc = " The section kind."] pub section : IndexSectionId , # [doc = " The base offset of the unit's contribution to the section."] pub offset : u32 , # [doc = " The size of the unit's contribution to the section."] pub size : u32 , }
    };
}

UnitIndexSection!()