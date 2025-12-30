// Generated macro for impl_75 (impl)
macro_rules! Depcrate_fork_by_errorimpl_75 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_75"}
// Dependencies: {}
impl < M , P0 , P1 , F > DryDataProvider < M > for ForkByErrorProvider < P0 , P1 , F > where M : DataMarker , P0 : DryDataProvider < M > , P1 : DryDataProvider < M > , F : ForkByErrorPredicate , { fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { let result = self . 0 . dry_load (req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . 2 . test (M :: INFO , Some (req) , err) => return Err (err) , _ => () , } ; self . 1 . dry_load (req) } }
};
}
