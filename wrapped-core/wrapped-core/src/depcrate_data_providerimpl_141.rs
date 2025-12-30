// Generated macro for impl_141 (impl)
macro_rules! Depcrate_data_providerimpl_141 {
() => {
// Module: crate::data_provider
// Provides: {"impl_141"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > BoundDataProvider < M > for alloc :: boxed :: Box < P > where M : DynamicDataMarker , P : BoundDataProvider < M > + ? Sized , { # [inline] fn load_bound (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { (* * self) . load_bound (req) } # [inline] fn bound_marker (& self) -> DataMarkerInfo { (* * self) . bound_marker () } }
};
}
