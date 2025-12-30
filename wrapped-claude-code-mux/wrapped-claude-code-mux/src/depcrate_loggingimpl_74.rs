// Generated macro for impl_74 (impl)
macro_rules! Depcrate_loggingimpl_74 {
() => {
// Module: crate::logging
// Provides: {"impl_74"}
// Dependencies: {}
impl < S > Layer < S > for QueryableLogLayer where S : Subscriber + for < 'a > LookupSpan < 'a > , { fn on_event (& self , event : & Event < '_ > , _ctx : Context < '_ , S >) { let mut visitor = LogVisitor :: default () ; event . record (& mut visitor) ; if let Some (message) = visitor . message { let log_entry = LogEntry { timestamp : Utc :: now () , level : event . metadata () . level () . to_string () , target : event . metadata () . target () . to_string () , message , } ; if let Ok (mut buffer) = self . buffer . write () { buffer . push_back (log_entry . clone ()) ; if buffer . len () > 1000 { buffer . pop_front () ; } } if let Ok (mut file) = self . log_file . write () { if let Ok (json) = serde_json :: to_string (& log_entry) { let _ = writeln ! (file , "{}" , json) ; } } } } }
};
}
