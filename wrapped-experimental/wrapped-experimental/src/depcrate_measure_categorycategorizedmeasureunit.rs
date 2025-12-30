// Generated macro for CategorizedMeasureUnit (struct)
macro_rules! Depcrate_measure_categoryCategorizedMeasureUnit {
() => {
// Module: crate::measure::category
// Provides: {"CategorizedMeasureUnit"}
// Dependencies: {}
# [doc = " A [`MeasureUnit`] that is related to a specific category."] # [doc = ""] # [doc = " This is useful for type inference and for ensuring that the correct units are used."] pub struct CategorizedMeasureUnit < T : MeasureUnitCategory > { _category : PhantomData < T > , pub unit : MeasureUnit , }
};
}
