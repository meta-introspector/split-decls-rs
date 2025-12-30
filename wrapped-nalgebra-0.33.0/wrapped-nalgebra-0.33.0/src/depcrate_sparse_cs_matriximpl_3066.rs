// Generated macro for impl_3066 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3066 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3066"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim > CsStorage < T , R , C > for CsVecStorage < T , R , C > where DefaultAllocator : Allocator < C > , { # [inline] fn shape (& self) -> (R , C) { self . shape } # [inline] fn len (& self) -> usize { self . vals . len () } # [inline] fn row_index (& self , i : usize) -> usize { self . i [i] } # [inline] unsafe fn row_index_unchecked (& self , i : usize) -> usize { * self . i . get_unchecked (i) } # [inline] unsafe fn get_value_unchecked (& self , i : usize) -> & T { self . vals . get_unchecked (i) } # [inline] fn get_value (& self , i : usize) -> & T { & self . vals [i] } # [inline] fn column_range (& self , j : usize) -> Range < usize > { let end = if j + 1 == self . p . len () { self . len () } else { self . p [j + 1] } ; self . p [j] .. end } }
};
}
