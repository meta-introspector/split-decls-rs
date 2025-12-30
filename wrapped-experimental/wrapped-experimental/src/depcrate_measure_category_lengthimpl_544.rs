// Generated macro for impl_544 (impl)
macro_rules! Depcrate_measure_category_lengthimpl_544 {
() => {
// Module: crate::measure::category::length
// Provides: {"impl_544"}
// Dependencies: {}
impl Length { # [doc = " Returns a [`MeasureUnit`] representing length in meters."] pub fn meter () -> CategorizedMeasureUnit < Length > { CategorizedMeasureUnit { _category : core :: marker :: PhantomData , unit : MeasureUnit { id : Some ("meter") , single_units : SingleUnitVec :: One (SingleUnit { power : 1 , si_prefix : SiPrefix { power : 0 , base : Base :: Decimal , } , unit_id : * crate :: provider :: Baked :: UNIT_IDS_V1_UND_METER , }) , constant_denominator : 0 , } , } } }
};
}
