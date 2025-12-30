// Generated macro for impl_8 (impl)
macro_rules! Depcrate_eitherimpl_8 {
() => {
// Module: crate::either
// Provides: {"impl_8"}
// Dependencies: {}
impl < M : DynamicDataMarker , P0 : DynamicDryDataProvider < M > , P1 : DynamicDryDataProvider < M > > DynamicDryDataProvider < M > for EitherProvider < P0 , P1 > { # [inline] fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { use EitherProvider :: * ; match self { A (p) => p . dry_load_data (marker , req) , B (p) => p . dry_load_data (marker , req) , } } }
};
}
