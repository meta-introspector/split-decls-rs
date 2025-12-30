// Generated macro for Size (struct)
macro_rules! Depcrate_ser_flavorsSize {
() => {
// Module: crate::ser::flavors
// Provides: {"Size"}
// Dependencies: {}
# [doc = " The `Size` flavor is a measurement flavor, which accumulates the number of bytes needed to"] # [doc = " serialize the data."] # [doc = ""] # [doc = " ```"] # [doc = " use postcard2::{serialize_with_flavor, ser_flavors};"] # [doc = ""] # [doc = " let value = false;"] # [doc = " let size = serialize_with_flavor(&value, ser_flavors::Size::default()).unwrap();"] # [doc = ""] # [doc = " assert_eq!(size, 1);"] # [doc = " ```"] # [derive (Default)] pub struct Size { size : usize , }
};
}
