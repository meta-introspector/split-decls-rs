// Generated macro for impl_552 (impl)
macro_rules! Depcrate_measure_category_volumeimpl_552 {
() => {
// Module: crate::measure::category::volume
// Provides: {"impl_552"}
// Dependencies: {}
impl Volume { # [doc = " Returns a [`MeasureUnit`] representing volume in cubic meters."] pub fn cubic_meter () -> CategorizedMeasureUnit < Volume > { CategorizedMeasureUnit { _category : core :: marker :: PhantomData , unit : MeasureUnit { id : Some ("cubic-meter") , single_units : SingleUnitVec :: One (SingleUnit { power : 3 , si_prefix : SiPrefix { power : 0 , base : Base :: Decimal , } , unit_id : * crate :: provider :: Baked :: UNIT_IDS_V1_UND_METER , }) , constant_denominator : 0 , } , } } # [doc = " Returns a [`MeasureUnit`] representing volume in liters."] pub fn liter () -> CategorizedMeasureUnit < Volume > { CategorizedMeasureUnit { _category : core :: marker :: PhantomData , unit : MeasureUnit { id : Some ("liter") , single_units : SingleUnitVec :: One (SingleUnit { power : 1 , si_prefix : SiPrefix { power : 0 , base : Base :: Decimal , } , unit_id : * crate :: provider :: Baked :: UNIT_IDS_V1_UND_LITER , }) , constant_denominator : 0 , } , } } }
};
}
