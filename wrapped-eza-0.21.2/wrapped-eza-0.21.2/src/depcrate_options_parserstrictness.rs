// Generated macro for Strictness (enum)
macro_rules! Depcrate_options_parserStrictness {
() => {
// Module: crate::options::parser
// Provides: {"Strictness"}
// Dependencies: {}
# [doc = " Whether redundant arguments should be considered a problem."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum Strictness { # [doc = " Throw an error when an argument doesn’t do anything, either because"] # [doc = " it requires another argument to be specified, or because two conflict."] ComplainAboutRedundantArguments , # [doc = " Search the arguments list back-to-front, giving ones specified later"] # [doc = " in the list priority over earlier ones."] UseLastArguments , }
};
}
