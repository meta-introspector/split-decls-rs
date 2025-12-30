// Generated macro for Outcome (struct)
macro_rules! Depcrate_searchOutcome {
() => {
// Module: crate::search
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The result of a search, containing all matching attributes."] # [derive (Default , Clone)] pub struct Outcome { # [doc = " The list of all available attributes, by ascending order. Each slots index corresponds to an attribute with that order, i.e."] # [doc = " `arr[attr.id] = <attr info>`."] # [doc = ""] # [doc = " This list needs to be up-to-date with the search group so all possible attribute names are known."] matches_by_id : Vec < Slot > , # [doc = " A stack of attributes to use for processing attributes of matched patterns and for resolving their macros."] attrs_stack : SmallVec < (AttributeId , Assignment , Option < AttributeId >) , 8 > , # [doc = " A set of attributes we should limit ourselves to, or empty if we should fill in all attributes, made of"] selected : SmallVec < (KString , Option < AttributeId >) , AVERAGE_NUM_ATTRS > , # [doc = " storage for all patterns we have matched so far (in order to avoid referencing them, we copy them, but only once)."] patterns : RefMap < gix_glob :: Pattern > , # [doc = " storage for all assignments we have matched so far (in order to avoid referencing them, we copy them, but only once)."] assignments : RefMap < Assignment > , # [doc = " storage for all source paths we have matched so far (in order to avoid referencing them, we copy them, but only once)."] source_paths : RefMap < std :: path :: PathBuf > , # [doc = " The amount of attributes that still need to be set, or `None` if this outcome is consumed which means it"] # [doc = " needs to be re-initialized."] remaining : Option < usize > , }
};
}
