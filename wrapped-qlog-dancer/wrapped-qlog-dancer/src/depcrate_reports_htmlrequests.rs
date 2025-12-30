// Generated macro for requests (function)
macro_rules! Depcrate_reports_htmlrequests {
() => {
// Module: crate::reports::html
// Provides: {"requests"}
// Dependencies: {}
pub fn requests (log_file : & LogFileParseResult , config : & AppConfig) { let filename = format ! ("{}-reports/requests.html" , config . filename) ; let mut file = create_file_recursive (& filename) . unwrap () ; file . write_all (HTML_INCLUDES . as_bytes ()) . unwrap () ; file . write_all (REQUEST_TABLE_INIT_SCRIPT . as_bytes ()) . unwrap () ; file . write_all (SESSIONS_STYLES . as_bytes ()) . unwrap () ; file . write_all (r#"<html>
    <head><head>
    <body>
        <h1 class="center">Summary of All HTTP Requests</h1>
        <div class="center" id="loading">
            <strong>Loading data...</strong>
            <div class="spinner-border" role="status">
            </div>
        </div>
        <div class="center">
            <p>This page provides information about the requests & responses in a connection.</p>
            <p> Each individual table represents an HTTP session with a unique ID. Each session is bound
            to an SNI and has a version. There can be multiple connections to the same SNI depending on the
            client's behaviour.</p>
            <p>In each table, a single row represents a request & response. The columns expressing different properties:</p>
            <details>
                <Summary>Click to expand</Summary>
                <p>
                <ul>
                    <li><strong>ID</strong> - the stream ID of the request & response</li>
                    <li><strong>Method</strong> - the request Method</li>
                    <li><strong>Host</strong> - the request Host (or authority). Due to connection coalescing, this value can be dfifferent from the session SNI</li>
                    <li><strong>Path</strong> - the request Path</li>
                    <li><strong>Status</strong> - the response Status</li>
                    <li><strong>Response Content-Length</strong> - for downloads; the value of the response Content-Length, if any. A response can omit this header. </li>
                    <li><strong>Response Transferred</strong> - for downloads; the actual number of bytes of response that were received. This can be less than Response Content-Length, indicating that the request or connection was terminated early.</li>
                    <li><strong>Download Duration (d2d) (ms)</strong> - the time duration between receiving the first and last DATA frame. This can be 0 for various reasons.</li>
                    <li><strong>Download Rate (d2d) (Mbps)</strong> - the download rate, in megabits/s, between first and last DATA frames. This number has caveats - can be very high if data size or durations are small.</li>
                    <li><strong>Download Duration (h2d) (ms)</strong> - the time duration between receiving the first HEADERS and last DATA frame. This can be 0 for various reasons.</li>
                    <li><strong>Dowload Rate (h2d) (Mbps)</strong> - the download rate, in megabits/s, between first HEADERS and last DATA frames. This number has caveats - can be very high if data size or durations are small.</li>
                    <li><strong>Client Tx Hdr, Rx First Data</strong> - the duration between the client sending a HEADERS frame, and the first DATA frame being received. This is analagous to TTFB.</li>
                    <li><strong>Client Tx Hdr, Rx Last Data</strong> - the duration between the client sending a HEADERS frame, and the last DATA frame being received. This is analagous to TTLB.</li>
                    <li><strong>Request Content-Length</strong> - for uploads; the value of the request Content-Length, if any. A request can omit this header. </li>
                    <li><strong>Request Transferred</strong> - for uploads; the actual number of bytes of request that were sent. This can be less than Request Content-Length, indicating that the request or connection was terminated early.</li>
                    <li><strong>Upload Duration (ms)</strong> - the time duration between sending the first and last DATA frame. This can be 0 for various reasons.</li>
                    <li><strong>Upload Rate (Mbps)</strong> - the upload rate, in megabits/s, between first and last DATA frames. This number has caveats - can be very high if data size or durations are small.</li>
                    <li><strong>Client Priority Header</strong> - the value of the RFC 9218 request Priority header, if any.</li>
                    <li><strong>Server Priority Header</strong> - the value of the RFC 9218 response Priority header, if any.</li>
                    <li><strong>Reset Stream Sent</strong> - the value of the error code in a Reset Stream, if sent.</li>
                    <li><strong>Reset Stream Received</strong> - the value of the error code in a Reset Stream, if received.</li>
                    <li><strong>Stop Sending Sent</strong> - the value of the error code in a Stop Sending, if sent.</li>
                </ul>
                </p>
            </details>
        </div>
        <div id="tables" style="visibility: hidden;">
            "# . as_bytes () ,) . unwrap () ; for data in & log_file . data { file . write_all (format ! ("<h2 class=\"center\">Session ID: {:?}, {:?}, {:?}</h2>" , data . datastore . session_id . unwrap_or (- 1) , data . datastore . host . clone () . unwrap_or ("ERROR UNKNOWN" . to_string ()) , data . datastore . application_proto) . as_bytes () ,) . unwrap () ; let table : tabled :: builder :: Builder = request_timing_table (data , config) . unwrap () . into () ; let mut reqs = HtmlTable :: with_header (Vec :: < Vec < String > > :: from (table)) ; reqs . visit_mut (RequestTableDecorator { i : 0 }) ; file . write_all (inject_table_id_class (& reqs , None , Some ("log-dancer-table cell-border hover compact order-column" . to_string () ,) ,) . as_bytes () ,) . unwrap () ; } file . write_all (r#"
        </div>
    </body>
    <html>"# . as_bytes () ,) . unwrap () ; }
};
}
