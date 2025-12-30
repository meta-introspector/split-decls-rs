// Generated macro for stdout_handle (function)
macro_rules! Depcrate_walkstdout_handle {
() => {
// Module: crate::walk
// Provides: {"stdout_handle"}
// Dependencies: {}
# [doc = " Returns a handle to stdout for filtering search."] # [doc = ""] # [doc = " A handle is returned if and only if stdout is being redirected to a file."] # [doc = " The handle returned corresponds to that file."] # [doc = ""] # [doc = " This can be used to ensure that we do not attempt to search a file that we"] # [doc = " may also be writing to."] fn stdout_handle () -> Option < Handle > { let h = match Handle :: stdout () { Err (_) => return None , Ok (h) => h , } ; let md = match h . as_file () . metadata () { Err (_) => return None , Ok (md) => md , } ; if ! md . is_file () { return None ; } Some (h) }
};
}
