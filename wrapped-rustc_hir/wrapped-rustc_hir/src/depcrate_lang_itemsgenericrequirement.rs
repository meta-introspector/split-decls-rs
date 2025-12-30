// Generated macro for GenericRequirement (enum)
macro_rules! Depcrate_lang_itemsGenericRequirement {
() => {
// Module: crate::lang_items
// Provides: {"GenericRequirement"}
// Dependencies: {}
# [doc = " The requirement imposed on the generics of a lang item"] pub enum GenericRequirement { # [doc = " No restriction on the generics"] None , # [doc = " A minimum number of generics that is demanded on a lang item"] Minimum (usize) , # [doc = " The number of generics must match precisely as stipulated"] Exact (usize) , }
};
}
