// Generated macro for overview (function)
macro_rules! Depcrate_reports_htmloverview {
() => {
// Module: crate::reports::html
// Provides: {"overview"}
// Dependencies: {}
pub fn overview (log_file : & LogFileParseResult , config : & AppConfig) { let mut h2 = vec ! [] ; let mut quic = vec ! [] ; for data in & log_file . data { if let Some (h2_close) = & data . datastore . h2_session_close { h2 . push (h2_close) ; } if let Some (quic_close) = & data . datastore . quic_session_close { quic . push (quic_close) ; } } let filename = format ! ("{}-reports/overview.html" , config . filename) ; let mut file = create_file_recursive (& filename) . unwrap () ; file . write_all (HTML_INCLUDES . as_bytes ()) . unwrap () ; file . write_all (TABLE_INIT_SCRIPT . as_bytes ()) . unwrap () ; file . write_all (SESSIONS_STYLES . as_bytes ()) . unwrap () ; file . write_all (r#"<html>
    <head><head>
    <body>
        <div>
            <h1 class="center">Session Overview</h1>
            <p class="center">This page lists all the sessions (aka connections)
            that were present in a log file. It is possible to filter only specific
            SNIs for analysis using the qlog-dancer `--netlog-filter` option.</p>
            <p class ="center">Analysed session detailed information is presented in
            <a href="closures.html">session terminations</a>
            and <a href="requests.html">requests breakdown</a>.</p>
            "# . as_bytes ()) . unwrap () ; let all_table = HtmlTable :: with_header (Vec :: < Vec < String > > :: from (Table :: builder (log_file . details . sessions . values ()) ,)) ; file . write_all (inject_table_id_class (& all_table , Some ("all_sessions" . to_string ()) , Some ("log-dancer-table cell-border hover compact order-column" . to_string () ,) ,) . as_bytes () ,) . unwrap () ; file . write_all (r#"
        </div>
    </body>
    <html>"# . as_bytes () ,) . unwrap () ; }
};
}
