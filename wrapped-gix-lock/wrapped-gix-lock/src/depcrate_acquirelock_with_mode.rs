// Generated macro for lock_with_mode (function)
macro_rules! Depcrate_acquirelock_with_mode {
() => {
// Module: crate::acquire
// Provides: {"lock_with_mode"}
// Dependencies: {}
fn lock_with_mode < T > (resource : & Path , mode : Fail , boundary_directory : Option < PathBuf > , try_lock : & dyn Fn (& Path , ContainingDirectory , AutoRemove) -> std :: io :: Result < T > ,) -> Result < (PathBuf , T) , Error > { use std :: io :: ErrorKind :: * ; let (directory , cleanup) = dir_cleanup (boundary_directory) ; let lock_path = add_lock_suffix (resource) ; let mut attempts = 1 ; match mode { Fail :: Immediately => try_lock (& lock_path , directory , cleanup) , Fail :: AfterDurationWithBackoff (time) => { for wait in backoff :: Quadratic :: default_with_random () . until_no_remaining (time) { attempts += 1 ; match try_lock (& lock_path , directory , cleanup . clone ()) { Ok (v) => return Ok ((lock_path , v)) , # [cfg (windows)] Err (err) if err . kind () == AlreadyExists || err . kind () == PermissionDenied => { std :: thread :: sleep (wait) ; continue ; } # [cfg (not (windows))] Err (err) if err . kind () == AlreadyExists => { std :: thread :: sleep (wait) ; continue ; } Err (err) => return Err (Error :: from (err)) , } } try_lock (& lock_path , directory , cleanup) } } . map (| v | (lock_path , v)) . map_err (| err | match err . kind () { AlreadyExists => Error :: PermanentlyLocked { resource_path : resource . into () , mode , attempts , } , _ => Error :: Io (err) , }) }
};
}
