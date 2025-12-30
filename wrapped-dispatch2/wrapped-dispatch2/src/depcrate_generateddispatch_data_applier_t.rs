// Generated macro for dispatch_data_applier_t (type)
macro_rules! Depcrate_generateddispatch_data_applier_t {
() => {
// Module: crate::generated
// Provides: {"dispatch_data_applier_t"}
// Dependencies: {}
# [doc = " A block to be invoked for every contiguous memory region in a data object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `region`: A data object representing the current region."] # [doc = ""] # [doc = " Parameter `offset`: The logical offset of the current region to the start"] # [doc = " of the data object."] # [doc = ""] # [doc = " Parameter `buffer`: The location of the memory for the current region."] # [doc = ""] # [doc = " Parameter `size`: The size of the memory for the current region."] # [doc = ""] # [doc = " Returns: A Boolean indicating whether traversal should continue."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/dispatch/dispatch_data_applier_t?language=objc)"] # [cfg (feature = "block2")] pub type dispatch_data_applier_t = * mut block2 :: DynBlock < dyn Fn (NonNull < DispatchData > , usize , NonNull < c_void > , usize) -> bool > ;
};
}
