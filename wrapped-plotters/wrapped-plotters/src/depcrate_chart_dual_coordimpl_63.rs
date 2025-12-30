// Generated macro for impl_63 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_63 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT1 : CoordTranslate , XT , YT , SX : Ranged < ValueType = XT > , SY : Ranged < ValueType = YT > , > DualCoordChartContext < 'a , DB , CT1 , Cartesian2d < SX , SY > > where SX : ValueFormatter < XT > , SY : ValueFormatter < YT > , { # [doc = " Start configure the style for the secondary axes"] pub fn configure_secondary_axes < 'b > (& 'b mut self) -> SecondaryMeshStyle < 'a , 'b , SX , SY , DB > { SecondaryMeshStyle :: new (& mut self . secondary) } }
};
}
