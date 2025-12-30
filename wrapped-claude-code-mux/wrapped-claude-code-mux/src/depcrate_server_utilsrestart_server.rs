// Generated macro for restart_server (function)
macro_rules! Depcrate_server_utilsrestart_server {
() => {
// Module: crate::server::utils
// Provides: {"restart_server"}
// Dependencies: {}
# [doc = " Restart server automatically using shell script"] pub async fn restart_server (State (state) : State < Arc < AppState > >) -> Response { info ! ("🔄 Server restart requested via UI") ; let config_lock = state . config . read () . await ; let port = config_lock . server . port ; match create_and_execute_restart_script (port) { Ok (_) => { info ! ("✅ Restart script initiated") ; let response = Html ("<div class='px-4 py-3 rounded-xl bg-green-500/20 border border-green-500/50 text-foreground text-sm'><strong>✅ Server restarting...</strong><br/>Shutting down current instance and starting new one.</div>") . into_response () ; tokio :: spawn (async { tokio :: time :: sleep (tokio :: time :: Duration :: from_secs (1)) . await ; info ! ("Shutting down for restart...") ; std :: process :: exit (0) ; }) ; response } Err (e) => { error ! ("Failed to initiate restart: {}" , e) ; Html (format ! ("<div class='px-4 py-3 rounded-xl bg-red-500/20 border border-red-500/50 text-foreground text-sm'><strong>❌ Restart failed</strong><br/>Error: {}</div>" , e)) . into_response () } } }
};
}
