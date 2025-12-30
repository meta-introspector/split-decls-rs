// Generated macro for impl_178 (impl)
macro_rules! Depcrateimpl_178 {
() => {
// Module: crate
// Provides: {"impl_178"}
// Dependencies: {}
impl Configure < Axis > for Figure { type Properties = axis :: Properties ; # [doc = " Configures an axis"] fn configure < F > (& mut self , axis : Axis , configure : F) -> & mut Figure where F : FnOnce (& mut axis :: Properties) -> & mut axis :: Properties , { if self . axes . contains_key (axis) { configure (self . axes . get_mut (axis) . unwrap ()) ; } else { let mut properties = Default :: default () ; configure (& mut properties) ; self . axes . insert (axis , properties) ; } self } }
};
}
