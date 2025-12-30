// Generated macro for err (macro)
macro_rules! Depcrate_pg_types_ipnet_addresserr {
() => {
// Module: crate::pg::types::ipnet_address
// Provides: {"err"}
// Dependencies: {}
macro_rules ! err { () => { Err ("invalid network address format" . into ()) } ; ($ msg : expr) => { Err (format ! ("invalid network address format. {}" , $ msg) . into ()) } ; }
};
}
