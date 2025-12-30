// Generated macro for impl_9 (impl)
macro_rules! Depcrate_eitherimpl_9 {
() => {
// Module: crate::either
// Provides: {"impl_9"}
// Dependencies: {}
impl < M : DataMarker , P0 : DataProvider < M > , P1 : DataProvider < M > > DataProvider < M > for EitherProvider < P0 , P1 > { # [inline] fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { use EitherProvider :: * ; match self { A (p) => p . load (req) , B (p) => p . load (req) , } } }
};
}
