// Generated macro for impl_126 (impl)
macro_rules! Depcrate_filledcurveimpl_126 {
() => {
// Module: crate::filledcurve
// Provides: {"impl_126"}
// Dependencies: {}
impl < X , Y1 , Y2 > traits :: Plot < FilledCurve < X , Y1 , Y2 > > for Figure where X : IntoIterator , X :: Item : Data , Y1 : IntoIterator , Y1 :: Item : Data , Y2 : IntoIterator , Y2 :: Item : Data , { type Properties = Properties ; fn plot < F > (& mut self , fc : FilledCurve < X , Y1 , Y2 > , configure : F) -> & mut Figure where F : FnOnce (& mut Properties) -> & mut Properties , { let FilledCurve { x , y1 , y2 } = fc ; let mut props = Default :: default () ; configure (& mut props) ; let (x_factor , y_factor) = crate :: scale_factor (& self . axes , props . axes . unwrap_or (crate :: Axes :: BottomXLeftY)) ; let data = Matrix :: new (itertools :: izip ! (x , y1 , y2) , (x_factor , y_factor , y_factor)) ; self . plots . push (Plot :: new (data , & props)) ; self } }
};
}
