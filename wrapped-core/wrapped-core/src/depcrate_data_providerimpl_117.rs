// Generated macro for impl_117 (impl)
macro_rules! Depcrate_data_providerimpl_117 {
() => {
// Module: crate::data_provider
// Provides: {"impl_117"}
// Dependencies: {}
impl < M , P > DataProvider < M > for & P where M : DataMarker , P : DataProvider < M > + ? Sized , { # [inline] fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { (* self) . load (req) } }
};
}
