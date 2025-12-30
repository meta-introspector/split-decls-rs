// Generated macro for test (module)
macro_rules! Depcrate_bindtest {
() => {
// Module: crate::bind
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { ffi , Connection , Error , Result } ; # [test] fn invalid_name () -> Result < () > { let db = Connection :: open_in_memory () ? ; let mut stmt = db . prepare ("SELECT 1") ? ; let err = stmt . raw_bind_parameter (1 , 1) . unwrap_err () ; assert_eq ! (err . sqlite_error_code () , Some (ffi :: ErrorCode :: ParameterOutOfRange) ,) ; let err = stmt . raw_bind_parameter (":p1" , 1) . unwrap_err () ; assert_eq ! (err , Error :: InvalidParameterName (":p1" . to_owned ())) ; let err = stmt . raw_bind_parameter (c"x" , 1) . unwrap_err () ; assert_eq ! (err , Error :: InvalidParameterName ("x" . to_owned ())) ; Ok (()) } }
};
}
