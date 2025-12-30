// Generated macro for tests (module)
macro_rules! Depcrate_kqueuetests {
() => {
// Module: crate::kqueue
// Provides: {"tests"}
// Dependencies: {}
mod tests { use super :: { Config , KqueueWatcher , RecursiveMode } ; use crate :: Watcher ; use std :: error :: Error ; use std :: path :: PathBuf ; use std :: result :: Result ; # [test] fn test_remove_recursive () -> Result < () , Box < dyn Error > > { let path = PathBuf :: from ("src") ; let mut watcher = KqueueWatcher :: new (| event | println ! ("{:?}" , event) , Config :: default ()) ? ; watcher . watch (& path , RecursiveMode :: Recursive) ? ; let result = watcher . unwatch (& path) ; assert ! (result . is_ok () , "unwatch yielded error: {}" , result . unwrap_err ()) ; Ok (()) } }
};
}
