// Generated macro for LazyStateIDError (struct)
macro_rules! Depcrate_hybrid_idLazyStateIDError {
() => {
// Module: crate::hybrid::id
// Provides: {"LazyStateIDError"}
// Dependencies: {}
# [doc = " This error occurs when a lazy state ID could not be constructed."] # [doc = ""] # [doc = " This occurs when given an integer exceeding the maximum lazy state ID"] # [doc = " value."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements the `Error` trait."] # [derive (Clone , Debug , Eq , PartialEq)] pub (crate) struct LazyStateIDError { attempted : u64 , }
};
}
