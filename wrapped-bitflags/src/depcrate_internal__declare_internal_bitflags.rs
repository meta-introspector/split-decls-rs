// Generated macro for __declare_internal_bitflags (macro)
macro_rules! Depcrate_internal__declare_internal_bitflags {
() => {
// Module: crate::internal
// Provides: {"__declare_internal_bitflags"}
// Dependencies: {}
# [doc = " Declare the `bitflags`-facing bitflags struct."] # [doc = ""] # [doc = " This type is part of the `bitflags` crate's public API, but not part of the user's."] # [macro_export] # [doc (hidden)] macro_rules ! __declare_internal_bitflags { ($ vis : vis struct $ InternalBitFlags : ident : $ T : ty) => { # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] $ vis struct $ InternalBitFlags ($ T) ; } ; }
};
}
