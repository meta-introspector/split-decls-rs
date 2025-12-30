// Generated macro for RIGHT_OR_DUAL_JOINING_MASK (const)
macro_rules! DepcrateRIGHT_OR_DUAL_JOINING_MASK {
() => {
// Module: crate
// Provides: {"RIGHT_OR_DUAL_JOINING_MASK"}
// Dependencies: {}
# [doc = " Mask for checking for both left and dual joining."] pub const RIGHT_OR_DUAL_JOINING_MASK : JoiningTypeMask = JoiningTypeMask (joining_type_to_mask (icu_properties :: props :: JoiningType :: RightJoining) | joining_type_to_mask (icu_properties :: props :: JoiningType :: DualJoining) ,) ;
};
}
