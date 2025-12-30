// Generated macro for impl_74 (impl)
macro_rules! Depcrate_fork_by_errorimpl_74 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_74"}
// Dependencies: {}
impl < M , P0 , P1 , F > DataProvider < M > for ForkByErrorProvider < P0 , P1 , F > where M : DataMarker , P0 : DataProvider < M > , P1 : DataProvider < M > , F : ForkByErrorPredicate , { fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { let result = self . 0 . load (req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . 2 . test (M :: INFO , Some (req) , err) => return Err (err) , _ => () , } ; self . 1 . load (req) } }
};
}
