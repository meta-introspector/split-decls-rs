// Generated macro for closures (function)
macro_rules! Depcrate_reports_htmlclosures {
() => {
// Module: crate::reports::html
// Provides: {"closures"}
// Dependencies: {}
pub fn closures (log_file : & LogFileParseResult , config : & AppConfig) { let mut h2 = vec ! [] ; let mut quic = vec ! [] ; for data in & log_file . data { if let Some (h2_close) = & data . datastore . h2_session_close { h2 . push (h2_close) ; } if let Some (quic_close) = & data . datastore . quic_session_close { quic . push (quic_close) ; } } let filename = format ! ("{}-reports/closures.html" , config . filename) ; let mut file = create_file_recursive (& filename) . unwrap () ; file . write_all (HTML_INCLUDES . as_bytes ()) . unwrap () ; file . write_all (TABLE_INIT_SCRIPT . as_bytes ()) . unwrap () ; file . write_all (SESSIONS_STYLES . as_bytes ()) . unwrap () ; file . write_all (r#"<html>
    <head><head>
    <body>
        <div>
            <h1 class="center">Session Overview</h1>
            <p class="center">This page lists all the sessions (aka connections)
            that were present in a log file and filtered into analysis using the
            `--netlog-filter` option. Connections are split by HTTP version. A single SNI might
            have multiple sessions, and it might use multiple HTTP versions. The reason that
            each session is closed is also captured in the Error column (and subsequent columns).
            A log that is closed before a session is terminated will not show any value in the
            columns.</p>
            <h2 class="center">HTTP/2 Connections</h2>"# . as_bytes ()) . unwrap () ; let mut h2_html_table = HtmlTable :: with_header (Vec :: < Vec < String > > :: from (Table :: builder (h2))) ; h2_html_table . visit_mut (H2ClosureTableDecorator { i : 0 }) ; file . write_all (inject_table_id_class (& h2_html_table , Some ("h2_close" . to_string ()) , Some ("log-dancer-table cell-border hover compact order-column" . to_string () ,) ,) . as_bytes () ,) . unwrap () ; file . write_all (r#"
            <h2 class="center">HTTP/3 & QUIC Connections</h2>"# . as_bytes () ,) . unwrap () ; let mut quic_html_table = HtmlTable :: with_header (Vec :: < Vec < String > > :: from (Table :: builder (quic))) ; quic_html_table . visit_mut (QUICClosureTableDecorator { i : 0 }) ; file . write_all (inject_table_id_class (& quic_html_table , Some ("quic_close" . to_string ()) , Some ("log-dancer-table cell-border hover compact order-column" . to_string () ,) ,) . as_bytes () ,) . unwrap () ; file . write_all (r#"
        </div>
    </body>
    <html>"# . as_bytes () ,) . unwrap () ; }
};
}
