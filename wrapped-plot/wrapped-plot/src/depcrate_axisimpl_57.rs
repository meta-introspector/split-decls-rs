// Generated macro for impl_57 (impl)
macro_rules! Depcrate_axisimpl_57 {
() => {
// Module: crate::axis
// Provides: {"impl_57"}
// Dependencies: {}
impl < P , L > Set < TicLabels < P , L > > for Properties where L : IntoIterator , L :: Item : AsRef < str > , P : IntoIterator , P :: Item : Data , { # [doc = " Attaches labels to the tics of an axis"] fn set (& mut self , tics : TicLabels < P , L >) -> & mut Properties { let TicLabels { positions , labels } = tics ; let pairs = positions . into_iter () . zip (labels) . map (| (pos , label) | format ! ("'{}' {}" , label . as_ref () , pos . f64 ())) . collect :: < Vec < _ > > () ; if pairs . is_empty () { self . tics = None } else { self . tics = Some (pairs . join (", ")) ; } self } }
};
}
