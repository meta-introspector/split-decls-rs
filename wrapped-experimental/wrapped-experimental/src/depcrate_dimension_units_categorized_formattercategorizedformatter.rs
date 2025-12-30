// Generated macro for CategorizedFormatter (struct)
macro_rules! Depcrate_dimension_units_categorized_formatterCategorizedFormatter {
() => {
// Module: crate::dimension::units::categorized_formatter
// Provides: {"CategorizedFormatter"}
// Dependencies: {}
# [doc = " A [`CategorizedFormatter`] is used to format specific units."] # [doc = ""] # [doc = " This is useful for type inference and for ensuring that the correct units are used."] pub struct CategorizedFormatter < C : MeasureUnitCategory > { _category : PhantomData < C > , display_name : DataPayload < crate :: dimension :: provider :: units :: display_names :: UnitsDisplayNamesV1 > , decimal_formatter : DecimalFormatter , plural_rules : PluralRules , }
};
}
