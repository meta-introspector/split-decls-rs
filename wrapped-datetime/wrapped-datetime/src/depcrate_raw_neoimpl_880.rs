// Generated macro for impl_880 (impl)
macro_rules! Depcrate_raw_neoimpl_880 {
() => {
// Module: crate::raw::neo
// Provides: {"impl_880"}
// Dependencies: {}
impl ZonePatternSelectionData { pub (crate) fn new_with_skeleton (field_set : ZoneFieldSet) -> Self { let (symbol , length) = field_set . to_field () ; let pattern_item = PatternItem :: Field (Field { symbol : FieldSymbol :: TimeZone (symbol) , length , }) ; Self :: SinglePatternItem (field_set , pattern_item . to_unaligned ()) } # [doc = " Borrows a pattern containing all of the fields that need to be loaded."] # [inline] pub (crate) fn pattern_items_for_data_loading (& self) -> impl Iterator < Item = PatternItem > + '_ { let Self :: SinglePatternItem (_ , pattern_item) = self ; let pattern_item = PatternItem :: from_unaligned (* pattern_item) ; [pattern_item] . into_iter () } # [doc = " Borrows a resolved pattern based on the given datetime"] pub (crate) fn select (& self , _input : & DateTimeInputUnchecked) -> ZonePatternDataBorrowed < '_ > { let Self :: SinglePatternItem (_ , pattern_item) = self ; ZonePatternDataBorrowed :: SinglePatternItem (pattern_item) } }
};
}
