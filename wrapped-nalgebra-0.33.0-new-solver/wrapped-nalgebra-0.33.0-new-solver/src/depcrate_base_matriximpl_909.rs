// Generated macro for impl_909 (impl)
macro_rules! Depcrate_base_matriximpl_909 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_909"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim > UninitMatrix < T , R , C > where DefaultAllocator : Allocator < R , C > , { # [doc = " Assumes a matrix's entries to be initialized. This operation should be near zero-cost."] # [doc = ""] # [doc = " # Safety"] # [doc = " The user must make sure that every single entry of the buffer has been initialized,"] # [doc = " or Undefined Behavior will immediately occur."] # [inline (always)] pub unsafe fn assume_init (self) -> OMatrix < T , R , C > { OMatrix :: from_data (< DefaultAllocator as Allocator < R , C > > :: assume_init (self . data ,)) } }
};
}
