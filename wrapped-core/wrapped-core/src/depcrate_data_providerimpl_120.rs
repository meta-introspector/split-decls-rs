// Generated macro for impl_120 (impl)
macro_rules! Depcrate_data_providerimpl_120 {
() => {
// Module: crate::data_provider
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [cfg (feature = "alloc")] impl < M , P > DataProvider < M > for alloc :: sync :: Arc < P > where M : DataMarker , P : DataProvider < M > + ? Sized , { # [inline] fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { (* * self) . load (req) } }
};
}
