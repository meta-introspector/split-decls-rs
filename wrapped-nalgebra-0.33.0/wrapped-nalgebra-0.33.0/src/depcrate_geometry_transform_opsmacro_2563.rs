// Generated macro for macro_2563 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2563 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2563"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (Const < D >, U1) const D ; for C ; where Const < D >: DimNameAdd < U1 >, C : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , C , D >, rhs : SVector < T , D >, Output = SVector < T , D >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => { let transform = self . matrix () . fixed_view ::< D , D > (0 , 0) ; if C :: has_normalizer () { let normalizer = self . matrix () . fixed_view ::< 1 , D > (D , 0) ; let n = normalizer . tr_dot (rhs) ; if ! n . is_zero () { return transform * (rhs / n) ; } } transform * rhs } ;) ;
};
}
