// Generated macro for impl_349 (impl)
macro_rules! Depcrate_uuidimpl_349 {
() => {
// Module: crate::uuid
// Provides: {"impl_349"}
// Dependencies: {}
# [cfg (feature = "with-uuid")] impl From < Uuid > for CFUUID { fn from (uuid : Uuid) -> CFUUID { let b = uuid . as_bytes () ; let bytes = CFUUIDBytes { byte0 : b [0] , byte1 : b [1] , byte2 : b [2] , byte3 : b [3] , byte4 : b [4] , byte5 : b [5] , byte6 : b [6] , byte7 : b [7] , byte8 : b [8] , byte9 : b [9] , byte10 : b [10] , byte11 : b [11] , byte12 : b [12] , byte13 : b [13] , byte14 : b [14] , byte15 : b [15] , } ; unsafe { let uuid_ref = CFUUIDCreateFromUUIDBytes (kCFAllocatorDefault , bytes) ; TCFType :: wrap_under_create_rule (uuid_ref) } } }
};
}
