// Generated macro for impl_119 (impl)
macro_rules! Depcrate_data_providerimpl_119 {
() => {
// Module: crate::data_provider
// Provides: {"impl_119"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > DataProvider < M > for alloc :: rc :: Rc < P > where M : DataMarker , P : DataProvider < M > + ? Sized , { # [inline] fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { (* * self) . load (req) } }
};
}
