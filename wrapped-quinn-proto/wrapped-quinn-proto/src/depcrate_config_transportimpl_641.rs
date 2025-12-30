// Generated macro for impl_641 (impl)
macro_rules! Depcrate_config_transportimpl_641 {
() => {
// Module: crate::config::transport
// Provides: {"impl_641"}
// Dependencies: {}
# [cfg (feature = "qlog")] impl QlogConfig { # [doc = " Where to write a qlog `TraceSeq`"] pub fn writer (& mut self , writer : Box < dyn io :: Write + Send + Sync >) -> & mut Self { self . writer = Some (writer) ; self } # [doc = " Title to record in the qlog capture"] pub fn title (& mut self , title : Option < String >) -> & mut Self { self . title = title ; self } # [doc = " Description to record in the qlog capture"] pub fn description (& mut self , description : Option < String >) -> & mut Self { self . description = description ; self } # [doc = " Epoch qlog event times are recorded relative to"] pub fn start_time (& mut self , start_time : Instant) -> & mut Self { self . start_time = start_time ; self } # [doc = " Construct the [`QlogStream`] described by this configuration"] pub fn into_stream (self) -> Option < QlogStream > { use tracing :: warn ; let writer = self . writer ? ; let trace = qlog :: TraceSeq :: new (qlog :: VantagePoint { name : None , ty : qlog :: VantagePointType :: Unknown , flow : None , } , self . title . clone () , self . description . clone () , Some (qlog :: Configuration { time_offset : Some (0.0) , original_uris : None , }) , None ,) ; let mut streamer = QlogStreamer :: new (qlog :: QLOG_VERSION . into () , self . title , self . description , None , self . start_time , trace , qlog :: events :: EventImportance :: Core , writer ,) ; match streamer . start_log () { Ok (()) => Some (QlogStream (Arc :: new (Mutex :: new (streamer)))) , Err (e) => { warn ! ("could not initialize endpoint qlog streamer: {e}") ; None } } } }
};
}
