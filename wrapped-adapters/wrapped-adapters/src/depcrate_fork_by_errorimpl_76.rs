// Generated macro for impl_76 (impl)
macro_rules! Depcrate_fork_by_errorimpl_76 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_76"}
// Dependencies: {}
impl < M , P0 , P1 , F > DynamicDataProvider < M > for ForkByErrorProvider < P0 , P1 , F > where M : DynamicDataMarker , P0 : DynamicDataProvider < M > , P1 : DynamicDataProvider < M > , F : ForkByErrorPredicate , { fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { let result = self . 0 . load_data (marker , req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . 2 . test (marker , Some (req) , err) => return Err (err) , _ => () , } ; self . 1 . load_data (marker , req) } }
};
}
