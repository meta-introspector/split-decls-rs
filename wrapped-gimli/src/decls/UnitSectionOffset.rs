macro_rules! deps {
    () => {
        SectionId!();
    };
}

macro_rules! UnitSectionOffset {
    () => {
        deps!();
        # [doc = " An offset into the `.debug_info` or `.debug_types` sections."] # [doc = ""] # [doc = " This type does not store which section the offset applies to. You will need to either"] # [doc = " determine that from the context of its use, or store a [`SectionId`] along with it."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct UnitSectionOffset < T = usize > (pub T) ;
    };
}

UnitSectionOffset!();