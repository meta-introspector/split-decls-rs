// Generated macro for ImageSectionHeader (struct)
macro_rules! Depcrate_peImageSectionHeader {
() => {
// Module: crate::pe
// Provides: {"ImageSectionHeader"}
// Dependencies: {}
# [derive (Debug , Default , Clone , Copy)] # [repr (C)] pub struct ImageSectionHeader { pub name : [u8 ; IMAGE_SIZEOF_SHORT_NAME] , pub virtual_size : U32 < LE > , pub virtual_address : U32 < LE > , pub size_of_raw_data : U32 < LE > , pub pointer_to_raw_data : U32 < LE > , pub pointer_to_relocations : U32 < LE > , pub pointer_to_linenumbers : U32 < LE > , pub number_of_relocations : U16 < LE > , pub number_of_linenumbers : U16 < LE > , pub characteristics : U32 < LE > , }
};
}
