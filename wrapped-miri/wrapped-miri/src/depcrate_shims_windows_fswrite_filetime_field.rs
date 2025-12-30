// Generated macro for write_filetime_field (function)
macro_rules! Depcrate_shims_windows_fswrite_filetime_field {
() => {
// Module: crate::shims::windows::fs
// Provides: {"write_filetime_field"}
// Dependencies: {}
fn write_filetime_field < 'tcx > (cx : & mut MiriInterpCx < 'tcx > , val : & MPlaceTy < 'tcx > , name : & str , (low , high) : (u32 , u32) ,) -> InterpResult < 'tcx > { cx . write_int_fields_named (& [("dwLowDateTime" , low . into ()) , ("dwHighDateTime" , high . into ())] , & cx . project_field_named (val , name) ? ,) }
};
}
