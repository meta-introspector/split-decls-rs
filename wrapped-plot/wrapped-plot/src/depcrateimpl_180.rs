// Generated macro for impl_180 (impl)
macro_rules! Depcrateimpl_180 {
() => {
// Module: crate
// Provides: {"impl_180"}
// Dependencies: {}
impl Set < BoxWidth > for Figure { # [doc = " Changes the box width of all the box related plots (bars, candlesticks, etc)"] # [doc = ""] # [doc = " **Note** The default value is 0"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `width` is a negative value"] fn set (& mut self , width : BoxWidth) -> & mut Figure { let width = width . 0 ; assert ! (width >= 0.) ; self . box_width = Some (width) ; self } }
};
}
