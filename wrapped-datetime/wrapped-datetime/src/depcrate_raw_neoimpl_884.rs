// Generated macro for impl_884 (impl)
macro_rules! Depcrate_raw_neoimpl_884 {
() => {
// Module: crate::raw::neo
// Provides: {"impl_884"}
// Dependencies: {}
impl < 'a > ItemsAndOptions < 'a > { pub (crate) fn iter_items (self) -> impl Iterator < Item = PatternItem > + 'a { self . items . iter () . map (move | mut pattern_item | { # [expect (clippy :: single_match)] match & mut pattern_item { PatternItem :: Field (ref mut field) => { let alignment = self . alignment . unwrap_or_default () ; if matches ! (alignment , Alignment :: Column) && field . length == FieldLength :: One && matches ! (field . symbol , FieldSymbol :: Month (_) | FieldSymbol :: Day (_) | FieldSymbol :: Week (_) | FieldSymbol :: Hour (_)) { field . length = FieldLength :: Two ; } if let Some (hour_cycle) = self . hour_cycle { if let FieldSymbol :: Hour (_) = field . symbol { field . symbol = FieldSymbol :: Hour (hour_cycle) ; } } if let Some (subsecond_digits) = self . subsecond_digits { if matches ! (field . symbol , FieldSymbol :: Second (fields :: Second :: Second) | FieldSymbol :: DecimalSecond (_)) { field . symbol = FieldSymbol :: from_subsecond_digits (subsecond_digits) ; } } } _ => () , } pattern_item }) } }
};
}
