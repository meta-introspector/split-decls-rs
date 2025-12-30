// Generated macro for download_http_with_retries (function)
macro_rules! Depcrate_core_downloaddownload_http_with_retries {
() => {
// Module: crate::core::download
// Provides: {"download_http_with_retries"}
// Dependencies: {}
fn download_http_with_retries (host_target : TargetSelection , is_running_on_ci : bool , exec_ctx : & ExecutionContext , tempfile : & Path , url : & str , help_on_error : & str ,) { println ! ("downloading {url}") ; let mut curl = command ("curl") . allow_failure () ; curl . args (["--location" , "--speed-time" , "30" , "--speed-limit" , "10" , "--connect-timeout" , "30" , "--output" , tempfile . to_str () . unwrap () , "--continue-at" , "-" , "--retry" , "3" , "--show-error" , "--remote-time" , "--fail" ,]) ; if is_running_on_ci { curl . arg ("--silent") ; } else { curl . arg ("--progress-bar") ; } if curl_version (exec_ctx) >= semver :: Version :: new (7 , 71 , 0) { curl . arg ("--retry-all-errors") ; } curl . arg (url) ; if ! curl . run (exec_ctx) { if host_target . contains ("windows-msvc") { eprintln ! ("Fallback to PowerShell") ; for _ in 0 .. 3 { let powershell = command ("PowerShell.exe") . allow_failure () . args (["/nologo" , "-Command" , "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;" , & format ! ("(New-Object System.Net.WebClient).DownloadFile('{}', '{}')" , url , tempfile . to_str () . expect ("invalid UTF-8 not supported with powershell downloads") ,) ,]) . run_capture_stdout (exec_ctx) ; if powershell . is_failure () { return ; } eprintln ! ("\nspurious failure, trying again") ; } } if ! help_on_error . is_empty () { eprintln ! ("{help_on_error}") ; } crate :: exit ! (1) ; } }
};
}
