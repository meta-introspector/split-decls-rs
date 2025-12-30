// Generated macro for impl_536 (impl)
macro_rules! Depcrate_measure_category_areaimpl_536 {
() => {
// Module: crate::measure::category::area
// Provides: {"impl_536"}
// Dependencies: {}
impl Area { # [doc = " Returns a [`MeasureUnit`] representing area in square meters."] pub fn square_meter () -> CategorizedMeasureUnit < Area > { CategorizedMeasureUnit { _category : core :: marker :: PhantomData , unit : MeasureUnit { id : Some ("square-meter") , single_units : SingleUnitVec :: One (SingleUnit { power : 2 , si_prefix : SiPrefix { power : 0 , base : Base :: Decimal , } , unit_id : * crate :: provider :: Baked :: UNIT_IDS_V1_UND_METER , }) , constant_denominator : 0 , } , } } }
};
}
