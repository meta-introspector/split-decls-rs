// Generated macro for impl_348 (impl)
macro_rules! Depcrate_uuidimpl_348 {
() => {
// Module: crate::uuid
// Provides: {"impl_348"}
// Dependencies: {}
# [cfg (feature = "with-uuid")] impl From < CFUUID > for Uuid { fn from (val : CFUUID) -> Self { let b = unsafe { CFUUIDGetUUIDBytes (val . 0) } ; let bytes = [b . byte0 , b . byte1 , b . byte2 , b . byte3 , b . byte4 , b . byte5 , b . byte6 , b . byte7 , b . byte8 , b . byte9 , b . byte10 , b . byte11 , b . byte12 , b . byte13 , b . byte14 , b . byte15 ,] ; Uuid :: from_bytes (bytes) } }
};
}
