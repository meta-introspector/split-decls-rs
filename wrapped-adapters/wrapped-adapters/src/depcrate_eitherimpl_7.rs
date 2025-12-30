// Generated macro for impl_7 (impl)
macro_rules! Depcrate_eitherimpl_7 {
() => {
// Module: crate::either
// Provides: {"impl_7"}
// Dependencies: {}
impl < M : DynamicDataMarker , P0 : DynamicDataProvider < M > , P1 : DynamicDataProvider < M > > DynamicDataProvider < M > for EitherProvider < P0 , P1 > { # [inline] fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { use EitherProvider :: * ; match self { A (p) => p . load_data (marker , req) , B (p) => p . load_data (marker , req) , } } }
};
}
