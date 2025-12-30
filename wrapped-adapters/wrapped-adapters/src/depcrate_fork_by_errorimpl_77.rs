// Generated macro for impl_77 (impl)
macro_rules! Depcrate_fork_by_errorimpl_77 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_77"}
// Dependencies: {}
impl < M , P0 , P1 , F > DynamicDryDataProvider < M > for ForkByErrorProvider < P0 , P1 , F > where M : DynamicDataMarker , P0 : DynamicDryDataProvider < M > , P1 : DynamicDryDataProvider < M > , F : ForkByErrorPredicate , { fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { let result = self . 0 . dry_load_data (marker , req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . 2 . test (marker , Some (req) , err) => return Err (err) , _ => () , } ; self . 1 . dry_load_data (marker , req) } }
};
}
