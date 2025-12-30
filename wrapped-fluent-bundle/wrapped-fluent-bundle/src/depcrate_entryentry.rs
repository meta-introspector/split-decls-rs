// Generated macro for Entry (enum)
macro_rules! Depcrate_entryEntry {
() => {
// Module: crate::entry
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " The [`Entry`] stores indexes into the [`FluentBundle`]'s resources for Messages and Terms,"] # [doc = " and owns the [`Box`] pointers to the [`FluentFunction`]."] pub enum Entry { Message ((ResourceIdx , EntryIdx)) , Term ((ResourceIdx , EntryIdx)) , Function (FluentFunction) , }
};
}
