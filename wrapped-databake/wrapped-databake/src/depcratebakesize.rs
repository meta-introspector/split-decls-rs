// Generated macro for BakeSize (trait)
macro_rules! DepcrateBakeSize {
() => {
// Module: crate
// Provides: {"BakeSize"}
// Dependencies: {}
# [doc = " Allows returning the size of data borrowed by a baked struct."] pub trait BakeSize : Sized + Bake { # [doc = " Returns the size"] fn borrows_size (& self) -> usize ; }
};
}
