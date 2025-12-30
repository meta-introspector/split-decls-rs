// Generated macro for test (module)
macro_rules! Depcrate_uuidtest {
() => {
// Module: crate::uuid
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "with-uuid")] mod test { use super :: CFUUID ; use uuid :: Uuid ; # [test] fn uuid_conversion () { let cf_uuid = CFUUID :: new () ; let uuid : Uuid = cf_uuid . clone () . into () ; let converted = CFUUID :: from (uuid) ; assert_eq ! (cf_uuid , converted) ; } }
};
}
