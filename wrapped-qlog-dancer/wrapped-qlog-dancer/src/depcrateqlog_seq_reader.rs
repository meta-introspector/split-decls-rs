// Generated macro for qlog_seq_reader (function)
macro_rules! Depcrateqlog_seq_reader {
() => {
// Module: crate
// Provides: {"qlog_seq_reader"}
// Dependencies: {}
pub fn qlog_seq_reader (config : & AppConfig ,) -> Result < (QlogSeqReader < '_ > , LogFileDetails) , Box < dyn Error > > { let file = std :: fs :: File :: open (config . file . clone ()) ? ; let reader = BufReader :: new (file) ; let qlog_reader = QlogSeqReader :: new (Box :: new (reader)) . map_err (| e | { std :: io :: Error :: other (format ! ("problem reading file! {}" , e)) }) . unwrap () ; let log_file_details = LogFileDetails { log_version : qlog_reader . qlog . qlog_version . clone () , log_format : qlog_reader . qlog . qlog_format . clone () , qlog_vantage_point_type : VantagePointTypeShim { inner : qlog_reader . qlog . trace . vantage_point . ty . clone () , } , sessions : BTreeMap :: new () , } ; Ok ((qlog_reader , log_file_details)) }
};
}
