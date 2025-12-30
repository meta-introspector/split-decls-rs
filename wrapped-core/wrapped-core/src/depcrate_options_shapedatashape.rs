// Generated macro for DataShape (struct)
macro_rules! Depcrate_options_shapeDataShape {
() => {
// Module: crate::options::shape
// Provides: {"DataShape"}
// Dependencies: {}
# [doc = " Receiver for shape information within a struct or enum context. See `Shape` for more information"] # [doc = " on valid uses of shape validation."] # [derive (Debug , Clone , Default , PartialEq , Eq)] pub struct DataShape { # [doc = " The kind of shape being described. This can be `struct_` or `enum_`."] prefix : & 'static str , newtype : bool , named : bool , tuple : bool , unit : bool , any : bool , }
};
}
