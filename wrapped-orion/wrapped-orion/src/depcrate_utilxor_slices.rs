// Generated macro for xor_slices (macro)
macro_rules! Depcrate_utilxor_slices {
() => {
// Module: crate::util
// Provides: {"xor_slices"}
// Dependencies: {}
# [doc = " xor_slices!(src, destination): XOR $src into $destination slice."] # [doc = " Uses iter() and .zip(), so it short-circuits on the slice that has"] # [doc = " the smallest length."] macro_rules ! xor_slices { ($ src : expr , $ destination : expr) => { for (inplace , _src_elem) in $ destination . iter_mut () . zip ($ src . iter ()) { * inplace ^= _src_elem ; } } ; }
};
}
