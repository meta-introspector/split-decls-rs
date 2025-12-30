// Generated macro for impl_670 (impl)
macro_rules! Depcrate_base_cgimpl_670 {
() => {
// Module: crate::base::cg
// Provides: {"impl_670"}
// Dependencies: {}
impl < T : RealField , S : Storage < T , Const < 3 > , Const < 3 > > > SquareMatrix < T , Const < 3 > , S > { # [doc = " Transforms the given point, assuming the matrix `self` uses homogeneous coordinates."] # [inline] pub fn transform_point (& self , pt : & Point < T , 2 >) -> Point < T , 2 > { let transform = self . fixed_view :: < 2 , 2 > (0 , 0) ; let translation = self . fixed_view :: < 2 , 1 > (0 , 2) ; let normalizer = self . fixed_view :: < 1 , 2 > (2 , 0) ; let n = normalizer . tr_dot (& pt . coords) + unsafe { self . get_unchecked ((2 , 2)) . clone () } ; if ! n . is_zero () { (transform * pt + translation) / n } else { transform * pt + translation } } }
};
}
