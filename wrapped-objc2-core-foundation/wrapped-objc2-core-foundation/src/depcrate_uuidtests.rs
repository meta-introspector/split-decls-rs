// Generated macro for tests (module)
macro_rules! Depcrate_uuidtests {
() => {
// Module: crate::uuid
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: CFUUID ; # [test] fn eq () { let uuid0 = CFUUID :: from_uuid_bytes (None , [0 ; 16] . into ()) . unwrap () ; let uuid1 = CFUUID :: from_uuid_bytes (None , [1 ; 16] . into ()) . unwrap () ; assert_eq ! (uuid0 , uuid0) ; assert_ne ! (uuid0 , uuid1) ; } # [test] fn roundtrip () { let uuid = CFUUID :: new (None) . unwrap () ; assert_eq ! (uuid , uuid) ; let bytes = uuid . uuid_bytes () ; let same_uuid = CFUUID :: from_uuid_bytes (None , bytes) . unwrap () ; assert_eq ! (uuid , same_uuid) ; } }
};
}
