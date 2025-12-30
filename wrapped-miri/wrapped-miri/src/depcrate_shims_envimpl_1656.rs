// Generated macro for impl_1656 (impl)
macro_rules! Depcrate_shims_envimpl_1656 {
() => {
// Module: crate::shims::env
// Provides: {"impl_1656"}
// Dependencies: {}
impl < 'tcx > EnvVars < 'tcx > { pub (crate) fn init (ecx : & mut InterpCx < 'tcx , MiriMachine < 'tcx > > , config : & MiriConfig ,) -> InterpResult < 'tcx > { let mut env_vars = FxHashMap :: default () ; if ecx . machine . communicate () || ! config . forwarded_env_vars . is_empty () { for (name , value) in & config . env { let forward = ecx . machine . communicate () || config . forwarded_env_vars . iter () . any (| v | * * v == * name) ; if forward { env_vars . insert (OsString :: from (name) , OsString :: from (value)) ; } } } for (name , value) in & config . set_env_vars { env_vars . insert (OsString :: from (name) , OsString :: from (value)) ; } let env_vars = if ecx . target_os_is_unix () { EnvVars :: Unix (UnixEnvVars :: new (ecx , env_vars) ?) } else if ecx . tcx . sess . target . os == "windows" { EnvVars :: Windows (WindowsEnvVars :: new (ecx , env_vars) ?) } else { EnvVars :: Uninit } ; ecx . machine . env_vars = env_vars ; interp_ok (()) } pub (crate) fn unix (& self) -> & UnixEnvVars < 'tcx > { match self { EnvVars :: Unix (env) => env , _ => unreachable ! () , } } pub (crate) fn unix_mut (& mut self) -> & mut UnixEnvVars < 'tcx > { match self { EnvVars :: Unix (env) => env , _ => unreachable ! () , } } pub (crate) fn windows (& self) -> & WindowsEnvVars { match self { EnvVars :: Windows (env) => env , _ => unreachable ! () , } } pub (crate) fn windows_mut (& mut self) -> & mut WindowsEnvVars { match self { EnvVars :: Windows (env) => env , _ => unreachable ! () , } } }
};
}
