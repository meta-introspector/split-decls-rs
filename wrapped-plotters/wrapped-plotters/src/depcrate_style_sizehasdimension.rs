// Generated macro for HasDimension (trait)
macro_rules! Depcrate_style_sizeHasDimension {
() => {
// Module: crate::style::size
// Provides: {"HasDimension"}
// Dependencies: {}
# [doc = " The trait indicates that the type has a dimensional data."] # [doc = " This is the abstraction for the relative sizing model."] # [doc = " A relative sizing value is able to be converted into a concrete size"] # [doc = " when coupling with a type with `HasDimension` type."] pub trait HasDimension { # [doc = " Get the dimensional data for this object"] fn dim (& self) -> (u32 , u32) ; }
};
}
