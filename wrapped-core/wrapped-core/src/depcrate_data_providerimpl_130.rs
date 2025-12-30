// Generated macro for impl_130 (impl)
macro_rules! Depcrate_data_providerimpl_130 {
() => {
// Module: crate::data_provider
// Provides: {"impl_130"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > DynamicDataProvider < M > for alloc :: rc :: Rc < P > where M : DynamicDataMarker , P : DynamicDataProvider < M > + ? Sized , { # [inline] fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { (* * self) . load_data (marker , req) } }
};
}
