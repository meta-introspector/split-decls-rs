// Generated macro for commands (function)
macro_rules! Depcrate_unixcommands {
() => {
// Module: crate::unix
// Provides: {"commands"}
// Dependencies: {}
pub fn commands < T : AsRef < OsStr > > (path : T) -> Vec < Command > { let path = path . as_ref () ; let mut commands : Vec < (& str , Vec < & OsStr >) > = vec ! [] ; let wsl_path = wsl_path (path) ; if is_wsl :: is_wsl () { commands . push (("wslview" , vec ! [& wsl_path])) ; } commands . extend_from_slice (& [("xdg-open" , vec ! [& path]) , ("gio" , vec ! [OsStr :: new ("open") , path]) , ("gnome-open" , vec ! [path]) , ("kde-open" , vec ! [path]) ,]) ; commands . iter () . map (| (command , args) | { let mut cmd = Command :: new (command) ; cmd . args (args) ; cmd }) . collect () }
};
}
