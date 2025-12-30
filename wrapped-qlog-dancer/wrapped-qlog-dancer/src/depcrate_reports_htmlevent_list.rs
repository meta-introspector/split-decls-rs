// Generated macro for event_list (function)
macro_rules! Depcrate_reports_htmlevent_list {
() => {
// Module: crate::reports::html
// Provides: {"event_list"}
// Dependencies: {}
pub fn event_list (log_file : & LogFileParseResult , config : & AppConfig) { let filename = format ! ("{}-reports/event-list.html" , config . filename) ; let mut file = create_file_recursive (& filename) . unwrap () ; file . write_all (HTML_INCLUDES . as_bytes ()) . unwrap () ; file . write_all (TABLE_INIT_SCRIPT . as_bytes ()) . unwrap () ; file . write_all (SESSIONS_STYLES . as_bytes ()) . unwrap () ; file . write_all (r#"<html>
    <head><head>
    <body>
        <div>
            <h1 class="center">List of events</h1>
            <p class="center">This page lists all the events
            that were present in a log file.</p>
            "# . as_bytes () ,) . unwrap () ; for data in & log_file . data { match & data . raw { crate :: RawLogEvents :: QlogJson { events : _ } => println ! ("Support for event list of contained qlog is TODO") , crate :: RawLogEvents :: QlogJsonSeq { events } => { let table = event_list_html_from_sqlog (events) ; file . write_all (table . as_bytes ()) . unwrap () ; } , crate :: RawLogEvents :: Netlog => println ! ("Support for event list of netlog is TODO") , } } file . write_all (r#"
        </div>
    </body>
    <html>"# . as_bytes () ,) . unwrap () ; }
};
}
