// Generated macro for Bytes (struct)
macro_rules! Depcrate_build_bytesBytes {
() => {
// Module: crate::build::bytes
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " A byte slice."] # [doc = ""] # [doc = " Uses copy-on-write to avoid unnecessary allocations. The bytes can be"] # [doc = " accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the"] # [doc = " `to_mut` method."] # [doc = ""] # [doc = " Provides a `Debug` implementation that shows the first 8 bytes and the length."] # [derive (Default , Clone , PartialEq , Eq)] pub struct Bytes < 'a > (Cow < 'a , [u8] >) ;
};
}
