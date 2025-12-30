// Generated macro for impl_533 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_533 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_533"}
// Dependencies: {}
impl < I0 : Iterator + Clone , Size : SizeDesc , DB : DrawingBackend > Drawable < DB > for DashedPathElement < I0 , Size > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , ps : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { let mut start = match points . next () { Some (c) => to_f (c) , None => return Ok (()) , } ; let size = self . size . in_pixels (& ps) . max (0) as f32 ; if size == 0. { return Ok (()) ; } let spacing = self . spacing . in_pixels (& ps) . max (0) as f32 ; let mut dist = 0. ; let mut is_solid = true ; let mut queue = vec ! [to_i (start)] ; for curr in points { let end = to_f (curr) ; while start != end { let (dx , dy) = (end . 0 - start . 0 , end . 1 - start . 1) ; let d = dx . hypot (dy) ; let size = if is_solid { size } else { spacing } ; let left = size - dist ; if left < d { let t = left / d ; start = (start . 0 + dx * t , start . 1 + dy * t) ; dist += left ; } else { start = end ; dist += d ; } if is_solid { queue . push (to_i (start)) ; } if size <= dist { if is_solid { backend . draw_path (queue . drain (..) , & self . style) ? ; } else { queue . push (to_i (start)) ; } dist = 0. ; is_solid = ! is_solid ; } } } if queue . len () > 1 { backend . draw_path (queue , & self . style) ? ; } Ok (()) } }
};
}
