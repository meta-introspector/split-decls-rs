// Generated macro for skip_container_prefixes (function)
macro_rules! Depcrate_parseskip_container_prefixes {
() => {
// Module: crate::parse
// Provides: {"skip_container_prefixes"}
// Dependencies: {}
pub (crate) fn skip_container_prefixes (tree : & Tree < Item > , bytes : & [u8] , options : Options) -> usize { let mut line_start = LineStart :: new (bytes) ; let _ = scan_containers (tree , & mut line_start , options) ; line_start . bytes_scanned () }
};
}
