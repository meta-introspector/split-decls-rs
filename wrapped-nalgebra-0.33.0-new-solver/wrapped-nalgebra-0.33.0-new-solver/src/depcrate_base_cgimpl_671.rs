// Generated macro for impl_671 (impl)
macro_rules! Depcrate_base_cgimpl_671 {
() => {
// Module: crate::base::cg
// Provides: {"impl_671"}
// Dependencies: {}
impl < T : RealField , S : Storage < T , Const < 4 > , Const < 4 > > > SquareMatrix < T , Const < 4 > , S > { # [doc = " Transforms the given point, assuming the matrix `self` uses homogeneous coordinates."] # [inline] pub fn transform_point (& self , pt : & Point < T , 3 >) -> Point < T , 3 > { let transform = self . fixed_view :: < 3 , 3 > (0 , 0) ; let translation = self . fixed_view :: < 3 , 1 > (0 , 3) ; let normalizer = self . fixed_view :: < 1 , 3 > (3 , 0) ; let n = normalizer . tr_dot (& pt . coords) + unsafe { self . get_unchecked ((3 , 3)) . clone () } ; if ! n . is_zero () { (transform * pt + translation) / n } else { transform * pt + translation } } }
};
}
