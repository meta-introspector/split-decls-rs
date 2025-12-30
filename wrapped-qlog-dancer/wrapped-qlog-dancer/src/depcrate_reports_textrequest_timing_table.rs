// Generated macro for request_timing_table (function)
macro_rules! Depcrate_reports_textrequest_timing_table {
() => {
// Module: crate::reports::text
// Provides: {"request_timing_table"}
// Dependencies: {}
pub fn request_timing_table (lf : & LogFileData , config : & AppConfig ,) -> Option < Table > { let mut table = Table :: new (lf . datastore . http_requests . values ()) ; table . with (Modify :: new (Segment :: all ()) . with (Alignment :: right ())) ; if config . report_omit_upload { table . with (Disable :: column (ByColumnName :: new (CLIENT_CONTENT_LENGTH))) . with (Disable :: column (ByColumnName :: new (CLIENT_TRANSFERRED))) . with (Disable :: column (ByColumnName :: new (UPLOAD_TIME))) . with (Disable :: column (ByColumnName :: new (UPLOAD_RATE))) ; } match lf . datastore . vantage_point { crate :: datastore :: VantagePoint :: Client => { table . with (Disable :: column (ByColumnName :: new (SERVER_RX_HDR_TX_HDR))) . with (Disable :: column (ByColumnName :: new (SERVER_TX_HDR_TX_FIRST_HDR ,))) . with (Disable :: column (ByColumnName :: new (SERVER_TX_HDR_TX_LAST_HDR ,))) . with (Disable :: column (ByColumnName :: new (SERVER_TX_FIRST_DATA_TX_LAST_DATA ,))) ; } , crate :: datastore :: VantagePoint :: Server => { } , } if config . report_omit_priorities { table . with (Disable :: column (ByColumnName :: new (CLIENT_PRI))) . with (Disable :: column (ByColumnName :: new (SERVER_PRI))) ; } if config . report_text_csv { let style = Style :: empty () . vertical (',') ; table . with (style) ; } Some (table) }
};
}
