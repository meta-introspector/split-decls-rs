// Generated macro for SeekFrom (enum)
macro_rules! DepcrateSeekFrom {
() => {
// Module: crate
// Provides: {"SeekFrom"}
// Dependencies: {}
# [doc = " Enumeration of possible methods to seek within an I/O object."] # [doc = ""] # [doc = " This is the `embedded-io` equivalent of [`std::io::SeekFrom`]."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum SeekFrom { # [doc = " Sets the offset to the provided number of bytes."] Start (u64) , # [doc = " Sets the offset to the size of this object plus the specified number of bytes."] End (i64) , # [doc = " Sets the offset to the current position plus the specified number of bytes."] Current (i64) , }
};
}
