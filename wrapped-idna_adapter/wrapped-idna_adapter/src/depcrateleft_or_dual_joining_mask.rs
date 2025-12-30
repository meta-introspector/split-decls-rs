// Generated macro for LEFT_OR_DUAL_JOINING_MASK (const)
macro_rules! DepcrateLEFT_OR_DUAL_JOINING_MASK {
() => {
// Module: crate
// Provides: {"LEFT_OR_DUAL_JOINING_MASK"}
// Dependencies: {}
# [doc = " Mask for checking for both left and dual joining."] pub const LEFT_OR_DUAL_JOINING_MASK : JoiningTypeMask = JoiningTypeMask (joining_type_to_mask (icu_properties :: props :: JoiningType :: LeftJoining) | joining_type_to_mask (icu_properties :: props :: JoiningType :: DualJoining) ,) ;
};
}
