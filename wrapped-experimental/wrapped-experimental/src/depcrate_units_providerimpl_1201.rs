// Generated macro for impl_1201 (impl)
macro_rules! Depcrate_units_providerimpl_1201 {
() => {
// Module: crate::units::provider
// Provides: {"impl_1201"}
// Dependencies: {}
impl UnitsInfo < '_ > { # [doc = " Retrieves the conversion details associated with a specific unit_id."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " * `unit_id` - A unique identifier representing the unit to be located."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Some(&ConversionInfoULE)` - A reference to the conversion information if the unit_id is found."] # [doc = " * `None` - If the unit_id is not found."] pub fn conversion_info_by_unit_id (& self , unit_id : UnitID) -> Option < & ConversionInfoULE > { self . conversion_info . zvl_binary_search_by (| convert_unit | { convert_unit . unit_id . as_unsigned_int () . cmp (& unit_id) }) . ok () . map (| index | & self . conversion_info [index]) } }
};
}
