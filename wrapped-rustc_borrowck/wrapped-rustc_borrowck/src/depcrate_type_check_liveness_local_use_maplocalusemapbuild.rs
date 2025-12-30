// Generated macro for LocalUseMapBuild (struct)
macro_rules! Depcrate_type_check_liveness_local_use_mapLocalUseMapBuild {
() => {
// Module: crate::type_check::liveness::local_use_map
// Provides: {"LocalUseMapBuild"}
// Dependencies: {}
struct LocalUseMapBuild < 'me > { local_use_map : & 'me mut LocalUseMap , location_map : & 'me DenseLocationMap , locals_with_use_data : IndexVec < Local , bool > , }
};
}
