// Generated macro for with_detached (function)
macro_rules! Depcrate_windowswith_detached {
() => {
// Module: crate::windows
// Provides: {"with_detached"}
// Dependencies: {}
# [cfg (feature = "shellexecute-on-windows")] pub fn with_detached < T : AsRef < OsStr > > (path : T , app : impl Into < String >) -> std :: io :: Result < () > { let app = wide (app . into ()) ; let path = wide (path) ; let mut info = ffi :: SHELLEXECUTEINFOW { cbSize : std :: mem :: size_of :: < ffi :: SHELLEXECUTEINFOW > () as _ , nShow : ffi :: SW_SHOWNORMAL , lpFile : app . as_ptr () , lpParameters : path . as_ptr () , .. unsafe { std :: mem :: zeroed () } } ; unsafe { ShellExecuteExW (& mut info) } }
};
}
