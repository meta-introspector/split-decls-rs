// Generated macro for macro_2564 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2564 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2564"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (Const < D >, U1) const D ; for C ; where Const < D >: DimNameAdd < U1 >, C : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , C , D >, rhs : Point < T , D >, Output = Point < T , D >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => { let transform = self . matrix () . fixed_view ::< D , D > (0 , 0) ; let translation = self . matrix () . fixed_view ::< D , 1 > (0 , D) ; if C :: has_normalizer () { let normalizer = self . matrix () . fixed_view ::< 1 , D > (D , 0) ; # [allow (clippy :: suspicious_arithmetic_impl)] let n = normalizer . tr_dot (& rhs . coords) + unsafe { self . matrix () . get_unchecked ((D , D)) . clone () } ; if ! n . is_zero () { return (transform * rhs + translation) / n ; } } transform * rhs + translation } ;) ;
};
}
