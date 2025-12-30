// Generated macro for impl_494 (impl)
macro_rules! Depcrate_duration_formatterimpl_494 {
() => {
// Module: crate::duration::formatter
// Provides: {"impl_494"}
// Dependencies: {}
impl From < BaseStyle > for icu_list :: options :: ListFormatterOptions { fn from (style : BaseStyle) -> Self { let length = match style { BaseStyle :: Long => ListLength :: Wide , BaseStyle :: Short | BaseStyle :: Digital => ListLength :: Short , BaseStyle :: Narrow => ListLength :: Narrow , } ; Self :: default () . with_length (length) } }
};
}
