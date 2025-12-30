// Generated macro for UnitSectionOffset (struct)
macro_rules! Depcrate_commonUnitSectionOffset {
() => {
// Module: crate::common
// Provides: {"UnitSectionOffset"}
// Dependencies: {}
# [doc = " An offset into the `.debug_info` or `.debug_types` sections."] # [doc = ""] # [doc = " This type does not store which section the offset applies to. You will need to either"] # [doc = " determine that from the context of its use, or store a [`SectionId`] along with it."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct UnitSectionOffset < T = usize > (pub T) ;
};
}
