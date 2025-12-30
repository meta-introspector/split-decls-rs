// Generated macro for Visible (trait)
macro_rules! Depcrate_core_formatVisible {
() => {
// Module: crate::core::format
// Provides: {"Visible"}
// Dependencies: {}
# [doc = " This is the trait that all elements that can be arranged need to implement."] pub trait Visible { # [doc = " \\return the Position of the shape."] fn position (& self) -> Position ; # [doc = " \\return the mutable reference to the Position of the shape."] fn position_mut (& mut self) -> & mut Position ; # [doc = " Return true if the element is a connector."] fn is_connector (& self) -> bool ; # [doc = " Swap the coordinates of the location and size."] fn transpose (& mut self) ; # [doc = " Update the size of the shape."] fn resize (& mut self) ; }
};
}
