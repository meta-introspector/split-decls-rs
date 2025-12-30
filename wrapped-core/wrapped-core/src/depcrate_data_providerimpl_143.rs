// Generated macro for impl_143 (impl)
macro_rules! Depcrate_data_providerimpl_143 {
() => {
// Module: crate::data_provider
// Provides: {"impl_143"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [cfg (feature = "alloc")] impl < M , P > BoundDataProvider < M > for alloc :: sync :: Arc < P > where M : DynamicDataMarker , P : BoundDataProvider < M > + ? Sized , { # [inline] fn load_bound (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { (* * self) . load_bound (req) } # [inline] fn bound_marker (& self) -> DataMarkerInfo { (* * self) . bound_marker () } }
};
}
