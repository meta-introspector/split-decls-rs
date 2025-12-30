// Generated macro for impl_277 (impl)
macro_rules! Depcrate_pattern_namesimpl_277 {
() => {
// Module: crate::pattern::names
// Provides: {"impl_277"}
// Dependencies: {}
impl YearNameLength { pub (crate) fn to_attributes (self) -> & 'static DataMarkerAttributes { use marker_attrs :: Length ; let length = match self { YearNameLength :: Abbreviated => Length :: Abbr , YearNameLength :: Wide => Length :: Wide , YearNameLength :: Narrow => Length :: Narrow , } ; marker_attrs :: name_attr_for (marker_attrs :: Context :: Format , length) } pub (crate) fn from_field_length (field_length : FieldLength) -> Option < Self > { let field_length = field_length . numeric_to_abbr () ; match field_length { FieldLength :: Three => Some (YearNameLength :: Abbreviated) , FieldLength :: Four => Some (YearNameLength :: Wide) , FieldLength :: Five => Some (YearNameLength :: Narrow) , _ => None , } } # [doc = " Returns an [`ErrorField`] sufficient for error reporting."] pub (crate) fn to_approximate_error_field (self) -> ErrorField { let field_length = match self { YearNameLength :: Abbreviated => FieldLength :: Three , YearNameLength :: Wide => FieldLength :: Four , YearNameLength :: Narrow => FieldLength :: Five , } ; ErrorField (fields :: Field { symbol : FieldSymbol :: Era , length : field_length , }) } }
};
}
