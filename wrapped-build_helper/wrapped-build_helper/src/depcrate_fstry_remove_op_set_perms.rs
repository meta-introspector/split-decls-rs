// Generated macro for try_remove_op_set_perms (function)
macro_rules! Depcrate_fstry_remove_op_set_perms {
() => {
// Module: crate::fs
// Provides: {"try_remove_op_set_perms"}
// Dependencies: {}
fn try_remove_op_set_perms < 'p , Op > (mut op : Op , path : & 'p Path , metadata : Metadata) -> io :: Result < () > where Op : FnMut (& 'p Path) -> io :: Result < () > , { match op (path) { Ok (()) => Ok (()) , Err (e) if e . kind () == io :: ErrorKind :: PermissionDenied => { let mut perms = metadata . permissions () ; perms . set_readonly (false) ; fs :: set_permissions (path , perms) ? ; op (path) } Err (e) => Err (e) , } }
};
}
