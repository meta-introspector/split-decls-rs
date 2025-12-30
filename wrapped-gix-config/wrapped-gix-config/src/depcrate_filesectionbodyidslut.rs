// Generated macro for SectionBodyIdsLut (enum)
macro_rules! Depcrate_fileSectionBodyIdsLut {
() => {
// Module: crate::file
// Provides: {"SectionBodyIdsLut"}
// Dependencies: {}
# [doc = " All section body ids referred to by a section name."] # [doc = ""] # [doc = " Note that order in Vec matters as it represents the order"] # [doc = " of section ids with the matched section and name, and is used for precedence"] # [doc = " management."] # [derive (PartialEq , Eq , Clone , Debug)] pub (crate) enum SectionBodyIdsLut < 'a > { # [doc = " The list of section ids to use for obtaining the section body."] Terminal (Vec < SectionId >) , # [doc = " A hashmap from sub-section names to section ids."] NonTerminal (HashMap < Cow < 'a , BStr > , Vec < SectionId > >) , }
};
}
