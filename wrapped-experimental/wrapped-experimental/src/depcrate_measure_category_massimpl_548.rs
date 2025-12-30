// Generated macro for impl_548 (impl)
macro_rules! Depcrate_measure_category_massimpl_548 {
() => {
// Module: crate::measure::category::mass
// Provides: {"impl_548"}
// Dependencies: {}
impl Mass { # [doc = " Returns a [`MeasureUnit`] representing mass in grams."] pub fn gram () -> CategorizedMeasureUnit < Mass > { CategorizedMeasureUnit { _category : core :: marker :: PhantomData , unit : MeasureUnit { id : Some ("gram") , single_units : SingleUnitVec :: One (SingleUnit { power : 1 , si_prefix : SiPrefix { power : 0 , base : Base :: Decimal , } , unit_id : * crate :: provider :: Baked :: UNIT_IDS_V1_UND_GRAM , }) , constant_denominator : 0 , } , } } # [doc = " Returns a [`MeasureUnit`] representing mass in kilograms."] pub fn kilogram () -> CategorizedMeasureUnit < Mass > { CategorizedMeasureUnit { _category : core :: marker :: PhantomData , unit : MeasureUnit { id : Some ("kilogram") , single_units : SingleUnitVec :: One (SingleUnit { power : 1 , si_prefix : SiPrefix { power : 0 , base : Base :: Decimal , } , unit_id : * crate :: provider :: Baked :: UNIT_IDS_V1_UND_KILOGRAM , }) , constant_denominator : 0 , } , } } }
};
}
