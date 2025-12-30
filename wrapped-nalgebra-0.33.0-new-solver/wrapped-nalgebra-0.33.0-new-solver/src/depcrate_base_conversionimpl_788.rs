// Generated macro for impl_788 (impl)
macro_rules! Depcrate_base_conversionimpl_788 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_788"}
// Dependencies: {}
impl < 'a , T , R , C , RView , CView , RStride , CStride , S > From < & 'a mut Matrix < T , R , C , S > > for MatrixView < 'a , T , RView , CView , RStride , CStride > where R : Dim , C : Dim , RView : Dim , CView : Dim , RStride : Dim , CStride : Dim , S : RawStorage < T , R , C > , ShapeConstraint : DimEq < R , RView > + DimEq < C , CView > + DimEq < RStride , S :: RStride > + DimEq < CStride , S :: CStride > , { fn from (m : & 'a mut Matrix < T , R , C , S >) -> Self { let (row , col) = m . shape_generic () ; let rows_result = RView :: from_usize (row . value ()) ; let cols_result = CView :: from_usize (col . value ()) ; let (rstride , cstride) = m . strides () ; let rstride_result = RStride :: from_usize (rstride) ; let cstride_result = CStride :: from_usize (cstride) ; unsafe { let data = ViewStorage :: from_raw_parts (m . data . ptr () , (rows_result , cols_result) , (rstride_result , cstride_result) ,) ; Matrix :: from_data_statically_unchecked (data) } } }
};
}
