// Generated macro for impl_538 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_538 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_538"}
// Dependencies: {}
impl < I0 , Size , DB , Marker > Drawable < DB > for DottedPathElement < I0 , Size , Marker > where I0 : Iterator + Clone , Size : SizeDesc , DB : DrawingBackend , Marker : crate :: element :: IntoDynElement < 'static , DB , BackendCoord > , { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , ps : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { let mut shift = self . shift . in_pixels (& ps) . max (0) as f32 ; let mut start = match points . next () { Some (start_i) => { if shift == 0. { let mk = (self . func) (start_i) . into_dyn () ; mk . draw (mk . point_iter () . iter () . copied () , backend , ps) ? ; } to_f (start_i) } None => return Ok (()) , } ; let spacing = self . spacing . in_pixels (& ps) . max (0) as f32 ; let mut dist = 0. ; for curr in points { let end = to_f (curr) ; while start != end { let (dx , dy) = (end . 0 - start . 0 , end . 1 - start . 1) ; let d = dx . hypot (dy) ; let spacing = if shift == 0. { spacing } else { shift } ; let left = spacing - dist ; if left < d { let t = left / d ; start = (start . 0 + dx * t , start . 1 + dy * t) ; dist += left ; } else { start = end ; dist += d ; } if spacing <= dist { let mk = (self . func) (to_i (start)) . into_dyn () ; mk . draw (mk . point_iter () . iter () . copied () , backend , ps) ? ; shift = 0. ; dist = 0. ; } } } Ok (()) } }
};
}
