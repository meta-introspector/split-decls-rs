// Generated macro for impl_182 (impl)
macro_rules! Depcrateimpl_182 {
() => {
// Module: crate
// Provides: {"impl_182"}
// Dependencies: {}
impl Set < FontSize > for Figure { # [doc = " Changes the size of the font"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `size` is a non-positive value"] fn set (& mut self , size : FontSize) -> & mut Figure { let size = size . 0 ; assert ! (size >= 0.) ; self . font_size = Some (size) ; self } }
};
}
