// Generated macro for ImageBoundImportDescriptor (struct)
macro_rules! Depcrate_peImageBoundImportDescriptor {
() => {
// Module: crate::pe
// Provides: {"ImageBoundImportDescriptor"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageBoundImportDescriptor { pub time_date_stamp : U32 < LE > , pub offset_module_name : U16 < LE > , pub number_of_module_forwarder_refs : U16 < LE > , }
};
}
