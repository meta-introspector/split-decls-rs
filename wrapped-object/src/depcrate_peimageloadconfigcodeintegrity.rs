// Generated macro for ImageLoadConfigCodeIntegrity (struct)
macro_rules! Depcrate_peImageLoadConfigCodeIntegrity {
() => {
// Module: crate::pe
// Provides: {"ImageLoadConfigCodeIntegrity"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageLoadConfigCodeIntegrity { # [doc = " Flags to indicate if CI information is available, etc."] pub flags : U16 < LE > , # [doc = " 0xFFFF means not available"] pub catalog : U16 < LE > , pub catalog_offset : U32 < LE > , # [doc = " Additional bitmask to be defined later"] pub reserved : U32 < LE > , }
};
}
