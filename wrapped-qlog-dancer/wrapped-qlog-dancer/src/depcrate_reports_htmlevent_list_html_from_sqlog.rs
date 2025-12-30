// Generated macro for event_list_html_from_sqlog (function)
macro_rules! Depcrate_reports_htmlevent_list_html_from_sqlog {
() => {
// Module: crate::reports::html
// Provides: {"event_list_html_from_sqlog"}
// Dependencies: {}
pub fn event_list_html_from_sqlog (events : & [qlog :: reader :: Event]) -> String { let table = sqlog_event_list (events) ; let table = HtmlTable :: with_header (Vec :: < Vec < String > > :: from (table)) ; inject_table_id_class (& table , None , Some ("log-dancer-table cell-border hover compact order-column" . to_string () ,) ,) }
};
}
