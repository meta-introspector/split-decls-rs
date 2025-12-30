// Generated macro for download_gccjit (function)
macro_rules! Depcrate_configdownload_gccjit {
() => {
// Module: crate::config
// Provides: {"download_gccjit"}
// Dependencies: {}
fn download_gccjit (commit : & str , output_dir : & Path , tempfile_name : String , with_progress_bar : bool ,) -> Result < () , String > { let url = if std :: env :: consts :: OS == "linux" && std :: env :: consts :: ARCH == "x86_64" { format ! ("https://github.com/rust-lang/gcc/releases/download/master-{commit}/libgccjit.so") } else { eprintln ! ("\
Pre-compiled libgccjit.so not available for this os or architecture.
Please compile it yourself and update the `config.toml` file
to `download-gccjit = false` and set `gcc-path` to the appropriate directory.") ; return Err (String :: from ("no appropriate pre-compiled libgccjit.so available for download" ,)) ; } ; println ! ("Downloading `{url}`...") ; let mut ret = run_command_with_output (& [& "curl" , & "--speed-time" , & "30" , & "--speed-limit" , & "10" , & "--connect-timeout" , & "30" , & "-o" , & tempfile_name , & "--retry" , & "3" , & "-SRfL" , if with_progress_bar { & "--progress-bar" } else { & "-s" } , & url . as_str () ,] , Some (output_dir) ,) ; if ret . is_err () && cfg ! (windows) { eprintln ! ("Fallback to PowerShell") ; ret = run_command_with_output (& [& "PowerShell.exe" , & "/nologo" , & "-Command" , & "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;" , & format ! ("(New-Object System.Net.WebClient).DownloadFile('{url}', '{tempfile_name}')" ,) . as_str () ,] , Some (output_dir) ,) ; } ret }
};
}
