// Generated macro for impl_731 (impl)
macro_rules! Depcrate_base_construction_viewimpl_731 {
() => {
// Module: crate::base::construction_view
// Provides: {"impl_731"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim > MatrixView < 'a , T , R , C > { # [doc = " Creates, without bound-checking, a matrix view from an array and with dimensions specified by generic types instances."] # [doc = ""] # [doc = " # Safety"] # [doc = " This method is unsafe because the input data array is not checked to contain enough elements."] # [doc = " The generic types `R` and `C` can either be type-level integers or integers wrapped with `Dyn()`."] # [inline] pub unsafe fn from_slice_generic_unchecked (data : & 'a [T] , start : usize , nrows : R , ncols : C ,) -> Self { Self :: from_slice_with_strides_generic_unchecked (data , start , nrows , ncols , Const :: < 1 > , nrows ,) } # [doc = " Creates a matrix view from an array and with dimensions and strides specified by generic types instances."] # [doc = ""] # [doc = " Panics if the input data array dose not contain enough elements."] # [doc = " The generic types `R` and `C` can either be type-level integers or integers wrapped with `Dyn()`."] # [inline] pub fn from_slice_generic (data : & 'a [T] , nrows : R , ncols : C) -> Self { Self :: from_slice_with_strides_generic (data , nrows , ncols , Const :: < 1 > , nrows) } }
};
}
