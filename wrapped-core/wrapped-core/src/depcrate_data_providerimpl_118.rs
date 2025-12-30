// Generated macro for impl_118 (impl)
macro_rules! Depcrate_data_providerimpl_118 {
() => {
// Module: crate::data_provider
// Provides: {"impl_118"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > DataProvider < M > for alloc :: boxed :: Box < P > where M : DataMarker , P : DataProvider < M > + ? Sized , { # [inline] fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { (* * self) . load (req) } }
};
}
