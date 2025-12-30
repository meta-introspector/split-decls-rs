// Generated macro for Scanner (struct)
macro_rules! Depcrate_content_yaml_vendored_scannerScanner {
() => {
// Module: crate::content::yaml::vendored::scanner
// Provides: {"Scanner"}
// Dependencies: {}
# [derive (Debug)] pub struct Scanner < T > { rdr : T , mark : Marker , tokens : VecDeque < Token > , buffer : VecDeque < char > , error : Option < ScanError > , stream_start_produced : bool , stream_end_produced : bool , adjacent_value_allowed_at : usize , simple_key_allowed : bool , simple_keys : Vec < SimpleKey > , indent : isize , indents : Vec < isize > , flow_level : u8 , tokens_parsed : usize , token_available : bool , }
};
}
