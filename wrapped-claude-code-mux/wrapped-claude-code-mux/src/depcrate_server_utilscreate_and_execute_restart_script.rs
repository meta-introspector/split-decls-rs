// Generated macro for create_and_execute_restart_script (function)
macro_rules! Depcrate_server_utilscreate_and_execute_restart_script {
() => {
// Module: crate::server::utils
// Provides: {"create_and_execute_restart_script"}
// Dependencies: {}
# [doc = " Create and execute a shell script that waits for shutdown and restarts"] pub fn create_and_execute_restart_script (port : u16) -> std :: io :: Result < () > { use std :: process :: Command ; use std :: fs ; let exe_path = std :: env :: current_exe () ? ; let current_pid = std :: process :: id () ; info ! ("Creating restart script for PID: {} on port: {}" , current_pid , port) ; # [cfg (unix)] { let script_content = format ! (r"#!/bin/bash
# Wait for old process to exit
while kill -0 {} 2>/dev/null; do
    sleep 0.1
done
# Start new server
{} start --port {} > /dev/null 2>&1 &
" , current_pid , exe_path . display () , port) ; let script_path = "/tmp/ccm_restart.sh" ; fs :: write (script_path , script_content) ? ; # [cfg (unix)] { use std :: os :: unix :: fs :: PermissionsExt ; let mut perms = fs :: metadata (script_path) ? . permissions () ; perms . set_mode (0o755) ; fs :: set_permissions (script_path , perms) ? ; } Command :: new ("sh") . arg (script_path) . stdin (std :: process :: Stdio :: null ()) . stdout (std :: process :: Stdio :: null ()) . stderr (std :: process :: Stdio :: null ()) . spawn () ? ; info ! ("Restart script started") ; } # [cfg (windows)] { let script_content = format ! (r###"@echo off
:wait
tasklist /FI "PID eq {0}" 2>NUL | find /I /N "ccm.exe">NUL
if "%ERRORLEVEL%"=="0" (
    timeout /t 1 /nobreak > nul
    goto wait
)
start "" "{1}" start --port {2}"### , current_pid , exe_path . display () , port) ; let script_path = std :: env :: temp_dir () . join ("ccm_restart.bat") ; fs :: write (& script_path , script_content) ? ; Command :: new ("cmd") . args (& ["/C" , "start" , "/B" , script_path . to_str () . unwrap ()]) . spawn () ? ; } Ok (()) }
};
}
