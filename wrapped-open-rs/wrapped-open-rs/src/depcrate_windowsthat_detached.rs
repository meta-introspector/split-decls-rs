// Generated macro for that_detached (function)
macro_rules! Depcrate_windowsthat_detached {
() => {
// Module: crate::windows
// Provides: {"that_detached"}
// Dependencies: {}
# [cfg (feature = "shellexecute-on-windows")] pub fn that_detached < T : AsRef < OsStr > > (path : T) -> std :: io :: Result < () > { use std :: path :: Path ; let path = path . as_ref () ; let is_dir = std :: fs :: metadata (path) . map (| f | f . is_dir ()) . unwrap_or (false) ; if is_dir { let path = dunce :: simplified (Path :: new (path)) ; let path = wide (path) ; unsafe { ffi :: CoInitialize (std :: ptr :: null ()) } ; let folder = unsafe { ffi :: ILCreateFromPathW (path . as_ptr ()) } ; if unsafe { SHOpenFolderAndSelectItems (folder , Some (& [folder]) , 0) } . is_ok () { return Ok (()) ; } } ; let path = wide (path) ; let (verb , class) = if is_dir { (ffi :: EXPLORE , ffi :: FOLDER) } else { (std :: ptr :: null () , std :: ptr :: null ()) } ; let mut info = ffi :: SHELLEXECUTEINFOW { cbSize : std :: mem :: size_of :: < ffi :: SHELLEXECUTEINFOW > () as _ , nShow : ffi :: SW_SHOWNORMAL , lpVerb : verb , lpClass : class , lpFile : path . as_ptr () , .. unsafe { std :: mem :: zeroed () } } ; unsafe { ShellExecuteExW (& mut info) } }
};
}
