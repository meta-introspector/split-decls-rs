// Generated macro for impl_622 (impl)
macro_rules! Depcrate_measure_single_unit_vecimpl_622 {
() => {
// Module: crate::measure::single_unit_vec
// Provides: {"impl_622"}
// Dependencies: {}
impl SingleUnitVec { # [doc = " Returns a slice of the [`SingleUnit`] instances contained"] # [doc = " within the [`SingleUnitVec`]."] pub (crate) fn as_slice (& self) -> & [SingleUnit] { match self { SingleUnitVec :: Empty => & [] , SingleUnitVec :: One (unit) => core :: slice :: from_ref (unit) , SingleUnitVec :: Two (units) => & units [..] , SingleUnitVec :: Multi (units) => & units [..] , } } pub (crate) fn push (& mut self , input_unit : SingleUnit) { match self { SingleUnitVec :: Empty => * self = SingleUnitVec :: One (input_unit) , SingleUnitVec :: One (current_unit) => { * self = SingleUnitVec :: Two ([* current_unit , input_unit]) } SingleUnitVec :: Two (current_units) => { * self = SingleUnitVec :: Multi (vec ! [current_units [0] , current_units [1] , input_unit]) ; } SingleUnitVec :: Multi (current_units) => current_units . push (input_unit) , } } }
};
}
