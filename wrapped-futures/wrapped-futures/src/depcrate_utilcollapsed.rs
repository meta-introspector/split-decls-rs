// Generated macro for Collapsed (enum)
macro_rules! Depcrate_utilCollapsed {
() => {
// Module: crate::util
// Provides: {"Collapsed"}
// Dependencies: {}
pub enum Collapsed < T : Future > { Start (T) , Tail (Box < Future < Item = T :: Item , Error = T :: Error > >) , }
};
}
