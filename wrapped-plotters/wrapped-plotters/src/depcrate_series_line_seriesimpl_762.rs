// Generated macro for impl_762 (impl)
macro_rules! Depcrate_series_line_seriesimpl_762 {
() => {
// Module: crate::series::line_series
// Provides: {"impl_762"}
// Dependencies: {}
impl < DB : DrawingBackend , Coord : Clone + 'static > Iterator for LineSeries < DB , Coord > { type Item = DynElement < 'static , DB , Coord > ; fn next (& mut self) -> Option < Self :: Item > { if ! self . data . is_empty () { if self . point_size > 0 && self . point_idx < self . data . len () { let idx = self . point_idx ; self . point_idx += 1 ; return Some (Circle :: new (self . data [idx] . clone () , self . point_size , self . style) . into_dyn () ,) ; } let mut data = vec ! [] ; std :: mem :: swap (& mut self . data , & mut data) ; Some (PathElement :: new (data , self . style) . into_dyn ()) } else { None } } }
};
}
