// Generated macro for system_time_from_nanos (function)
macro_rules! Depcratesystem_time_from_nanos {
() => {
// Module: crate
// Provides: {"system_time_from_nanos"}
// Dependencies: {}
fn system_time_from_nanos < 'de , D > (deserializer : D) -> Result < SystemTime , D :: Error > where D : Deserializer < 'de > , { let duration_from_epoch = Duration :: from_nanos (u64 :: deserialize (deserializer) ?) ; Ok (UNIX_EPOCH . checked_add (duration_from_epoch) . expect ("a time that can be represented as SystemTime")) }
};
}
