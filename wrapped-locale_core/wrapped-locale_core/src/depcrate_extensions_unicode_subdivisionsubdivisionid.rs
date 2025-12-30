// Generated macro for SubdivisionId (struct)
macro_rules! Depcrate_extensions_unicode_subdivisionSubdivisionId {
() => {
// Module: crate::extensions::unicode::subdivision
// Provides: {"SubdivisionId"}
// Dependencies: {}
# [doc = " A Subivision Id as defined in [`Unicode Locale Identifier`]."] # [doc = ""] # [doc = " Subdivision Id is used in [`Unicode`] extensions:"] # [doc = "  * `rg` - Regional Override"] # [doc = "  * `sd` - Regional Subdivision"] # [doc = ""] # [doc = " In both cases the subdivision is composed of a [`Region`] and a [`SubdivisionSuffix`] which represents"] # [doc = " different meaning depending on the key."] # [doc = ""] # [doc = " [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/tr35.html#unicode_subdivision_id"] # [doc = " [`Unicode`]: crate::extensions::unicode::Unicode"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::{"] # [doc = "     extensions::unicode::{subdivision_suffix, SubdivisionId},"] # [doc = "     subtags::region,"] # [doc = " };"] # [doc = ""] # [doc = " let ss = subdivision_suffix!(\"zzzz\");"] # [doc = " let region = region!(\"gb\");"] # [doc = ""] # [doc = " let si = SubdivisionId::new(region, ss);"] # [doc = ""] # [doc = " assert_eq!(si.to_string(), \"gbzzzz\");"] # [doc = " ```"] # [derive (Debug , PartialEq , Eq , Clone , Hash , PartialOrd , Ord , Copy)] # [non_exhaustive] pub struct SubdivisionId { # [doc = " A region field of a Subdivision Id."] pub region : Region , # [doc = " A subdivision suffix field of a Subdivision Id."] pub suffix : SubdivisionSuffix , }
};
}
