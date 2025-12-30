// Generated macro for DeriveInputShapeSet (struct)
macro_rules! Depcrate_options_shapeDeriveInputShapeSet {
() => {
// Module: crate::options::shape
// Provides: {"DeriveInputShapeSet"}
// Dependencies: {}
# [doc = " Receiver struct for shape validation. Shape validation allows a deriving type"] # [doc = " to declare that it only accepts - for example - named structs, or newtype enum"] # [doc = " variants."] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[ignore(any, struct_named, enum_newtype)]"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct DeriveInputShapeSet { enum_values : DataShape , struct_values : DataShape , any : bool , }
};
}
