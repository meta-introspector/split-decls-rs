// Generated macro for IMAGE_COR20_HEADER (struct)
macro_rules! Depcrate_bindingsIMAGE_COR20_HEADER {
() => {
// Module: crate::bindings
// Provides: {"IMAGE_COR20_HEADER"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct IMAGE_COR20_HEADER { pub cb : u32 , pub MajorRuntimeVersion : u16 , pub MinorRuntimeVersion : u16 , pub MetaData : IMAGE_DATA_DIRECTORY , pub Flags : u32 , pub Anonymous : IMAGE_COR20_HEADER_0 , pub Resources : IMAGE_DATA_DIRECTORY , pub StrongNameSignature : IMAGE_DATA_DIRECTORY , pub CodeManagerTable : IMAGE_DATA_DIRECTORY , pub VTableFixups : IMAGE_DATA_DIRECTORY , pub ExportAddressTableJumps : IMAGE_DATA_DIRECTORY , pub ManagedNativeHeader : IMAGE_DATA_DIRECTORY , }
};
}
