// Generated macro for SelectedTab (enum)
macro_rules! DepcrateSelectedTab {
() => {
// Module: crate
// Provides: {"SelectedTab"}
// Dependencies: {}
# [doc = " Tabs for the different layouts"] # [doc = ""] # [doc = " Note: the order of the variants will determine the order of the tabs this uses several derive"] # [doc = " macros from the `strum` crate to make it easier to iterate over the variants."] # [doc = " (`FromRepr`,`Display`,`EnumIter`)."] # [derive (Default , Debug , Copy , Clone , PartialEq , Eq , FromRepr , Display , EnumIter)] enum SelectedTab { # [default] Legacy , Start , Center , End , SpaceAround , SpaceEvenly , SpaceBetween , }
};
}
