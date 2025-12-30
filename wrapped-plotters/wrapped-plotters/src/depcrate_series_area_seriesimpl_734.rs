// Generated macro for impl_734 (impl)
macro_rules! Depcrate_series_area_seriesimpl_734 {
() => {
// Module: crate::series::area_series
// Provides: {"impl_734"}
// Dependencies: {}
impl < DB : DrawingBackend , X : Clone + 'static , Y : Clone + 'static > Iterator for AreaSeries < DB , X , Y > { type Item = DynElement < 'static , DB , (X , Y) > ; fn next (& mut self) -> Option < Self :: Item > { if self . state == 0 { let mut data : Vec < _ > = self . data . clone () ; if ! data . is_empty () { data . push ((data [data . len () - 1] . 0 . clone () , self . baseline . clone ())) ; data . push ((data [0] . 0 . clone () , self . baseline . clone ())) ; } self . state = 1 ; Some (Polygon :: new (data , self . area_style) . into_dyn ()) } else if self . state == 1 { let data : Vec < _ > = self . data . clone () ; self . state = 2 ; Some (PathElement :: new (data , self . border_style) . into_dyn ()) } else { None } } }
};
}
