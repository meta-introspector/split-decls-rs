// Generated macro for Dim (trait)
macro_rules! Depcrate_base_dimensionDim {
() => {
// Module: crate::base::dimension
// Provides: {"Dim"}
// Dependencies: {}
# [doc = " Trait implemented by any type that can be used as a dimension. This includes type-level"] # [doc = " integers and `Dyn` (for dimensions not known at compile-time)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Hoists integers to the type level, including binary operations."] pub unsafe trait Dim : Any + Debug + Copy + PartialEq + Send + Sync { # [inline (always)] fn is < D : Dim > () -> bool { TypeId :: of :: < Self > () == TypeId :: of :: < D > () } # [doc = " Gets the compile-time value of `Self`. Returns `None` if it is not known, i.e., if `Self ="] # [doc = " Dyn`."] fn try_to_usize () -> Option < usize > ; # [doc = " Gets the run-time value of `self`. For type-level integers, this is the same as"] # [doc = " `Self::try_to_usize().unwrap()`."] fn value (& self) -> usize ; # [doc = " Builds an instance of `Self` from a run-time value. Panics if `Self` is a type-level"] # [doc = " integer and `dim != Self::try_to_usize().unwrap()`."] fn from_usize (dim : usize) -> Self ; }
};
}
