// Generated macro for __declare_public_bitflags (macro)
macro_rules! Depcrate_public__declare_public_bitflags {
() => {
// Module: crate::public
// Provides: {"__declare_public_bitflags"}
// Dependencies: {}
# [doc = " Declare the user-facing bitflags struct."] # [doc = ""] # [doc = " This type is guaranteed to be a newtype with a `bitflags`-facing type as its single field."] # [macro_export] # [doc (hidden)] macro_rules ! __declare_public_bitflags { ($ (# [$ outer : meta]) * $ vis : vis struct $ PublicBitFlags : ident) => { $ (# [$ outer]) * $ vis struct $ PublicBitFlags (<$ PublicBitFlags as $ crate :: __private :: PublicFlags >:: Internal) ; } ; }
};
}
