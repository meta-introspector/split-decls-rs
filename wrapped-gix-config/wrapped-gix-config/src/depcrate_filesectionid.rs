// Generated macro for SectionId (struct)
macro_rules! Depcrate_fileSectionId {
() => {
// Module: crate::file
// Provides: {"SectionId"}
// Dependencies: {}
# [doc = " The section ID is a monotonically increasing ID used to refer to section bodies."] # [doc = " This value does not imply any ordering between sections, as new sections"] # [doc = " with higher section IDs may be in between lower ID sections after `File` mutation."] # [doc = ""] # [doc = " We need to use a section id because `git-config` permits sections with"] # [doc = " identical names, making it ambiguous when used in maps, for instance."] # [doc = ""] # [doc = " This id guaranteed to be unique, but not guaranteed to be compact. In other"] # [doc = " words, it's possible that a section may have an ID of 3 but the next section"] # [doc = " has an ID of 5 as 4 was deleted."] # [derive (PartialEq , Eq , Hash , Copy , Clone , PartialOrd , Ord , Debug)] pub struct SectionId (pub (crate) usize) ;
};
}
