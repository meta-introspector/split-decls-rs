// Generated macro for impl_762 (impl)
macro_rules! Depcrate_base_conversionimpl_762 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_762"}
// Dependencies: {}
impl < T1 , T2 , R1 , C1 , R2 , C2 > SubsetOf < OMatrix < T2 , R2 , C2 > > for OMatrix < T1 , R1 , C1 > where R1 : Dim , C1 : Dim , R2 : Dim , C2 : Dim , T1 : Scalar , T2 : Scalar + SupersetOf < T1 > , DefaultAllocator : Allocator < R2 , C2 > + Allocator < R1 , C1 > + SameShapeAllocator < R1 , C1 , R2 , C2 > , ShapeConstraint : SameNumberOfRows < R1 , R2 > + SameNumberOfColumns < C1 , C2 > , { # [inline] fn to_superset (& self) -> OMatrix < T2 , R2 , C2 > { let (nrows , ncols) = self . shape () ; let nrows2 = R2 :: from_usize (nrows) ; let ncols2 = C2 :: from_usize (ncols) ; let mut res = Matrix :: uninit (nrows2 , ncols2) ; for i in 0 .. nrows { for j in 0 .. ncols { unsafe { * res . get_unchecked_mut ((i , j)) = MaybeUninit :: new (T2 :: from_subset (self . get_unchecked ((i , j)))) ; } } } unsafe { res . assume_init () } } # [inline] fn is_in_subset (m : & OMatrix < T2 , R2 , C2 >) -> bool { m . iter () . all (| e | e . is_in_subset ()) } # [inline] fn from_superset_unchecked (m : & OMatrix < T2 , R2 , C2 >) -> Self { let (nrows2 , ncols2) = m . shape () ; let nrows = R1 :: from_usize (nrows2) ; let ncols = C1 :: from_usize (ncols2) ; let mut res = Matrix :: uninit (nrows , ncols) ; for i in 0 .. nrows2 { for j in 0 .. ncols2 { unsafe { * res . get_unchecked_mut ((i , j)) = MaybeUninit :: new (m . get_unchecked ((i , j)) . to_subset_unchecked ()) } } } unsafe { res . assume_init () } } }
};
}
