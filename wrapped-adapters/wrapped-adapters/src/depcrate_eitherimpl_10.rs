// Generated macro for impl_10 (impl)
macro_rules! Depcrate_eitherimpl_10 {
() => {
// Module: crate::either
// Provides: {"impl_10"}
// Dependencies: {}
impl < M : DataMarker , P0 : DryDataProvider < M > , P1 : DryDataProvider < M > > DryDataProvider < M > for EitherProvider < P0 , P1 > { # [inline] fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { use EitherProvider :: * ; match self { A (p) => p . dry_load (req) , B (p) => p . dry_load (req) , } } }
};
}
