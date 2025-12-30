// Generated macro for reveal_in_windows_explorer (function)
macro_rules! Depcrate_linux_and_morereveal_in_windows_explorer {
() => {
// Module: crate::linux_and_more
// Provides: {"reveal_in_windows_explorer"}
// Dependencies: {}
# [cfg (all (feature = "reveal" , target_os = "linux"))] fn reveal_in_windows_explorer (path : & std :: path :: Path) -> Result < () , OpenError > { let converted_path = crate :: wsl_to_windows_path (path . as_os_str ()) ; let converted_path = converted_path . as_deref () ; let path = match converted_path { None => path , Some (x) => std :: path :: Path :: new (x) , } ; Command :: new ("explorer.exe") . arg ("/select,") . arg (path) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . spawn () . map_err (| err | OpenError :: Spawn { cmds : "explorer.exe" . into () , source : err , }) ? ; Ok (()) }
};
}
