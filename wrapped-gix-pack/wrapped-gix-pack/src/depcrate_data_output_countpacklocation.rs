// Generated macro for PackLocation (enum)
macro_rules! Depcrate_data_output_countPackLocation {
() => {
// Module: crate::data::output::count
// Provides: {"PackLocation"}
// Dependencies: {}
# [doc = " Specifies how the pack location was handled during counting"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum PackLocation { # [doc = " We did not lookup this object"] NotLookedUp , # [doc = " The object was looked up and there may be a location in a pack, along with entry information"] LookedUp (Option < crate :: data :: entry :: Location >) , }
};
}
