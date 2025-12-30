// Generated macro for Data (enum)
macro_rules! Depcrate_serializeData {
() => {
// Module: crate::serialize
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Serialized database"] pub enum Data < 'conn > { # [doc = " Shared (SQLITE_SERIALIZE_NOCOPY) serialized database"] Shared (SharedData < 'conn >) , # [doc = " Owned serialized database"] Owned (OwnedData) , }
};
}
