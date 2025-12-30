// Generated macro for bytes_component_to_os_str (function)
macro_rules! Depcrate_stackbytes_component_to_os_str {
() => {
// Module: crate::stack
// Provides: {"bytes_component_to_os_str"}
// Dependencies: {}
fn bytes_component_to_os_str < 'a > (component : & 'a [u8] , path : & BStr ,) -> Option < Result < & 'a OsStr , to_normal_path_components :: Error > > { if component . is_empty () { return None ; } let component = match gix_path :: try_from_byte_slice (component . as_bstr ()) . map_err (| _ | to_normal_path_components :: Error :: IllegalUtf8) { Ok (c) => c , Err (err) => return Some (Err (err)) , } ; let component = component . components () . next () ? ; Some (component_to_os_str (component , gix_path :: try_from_byte_slice (path . as_ref ()) . ok () ? ,)) }
};
}
