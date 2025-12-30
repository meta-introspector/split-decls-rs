// Generated macro for process_stream (function)
macro_rules! Depcrate_recursiveprocess_stream {
() => {
// Module: crate::recursive
// Provides: {"process_stream"}
// Dependencies: {}
fn process_stream (stream : TcpStream , sender : & Sender < ClippyWarning > , options : & RecursiveOptions , seen : & Mutex < HashSet < DriverInfo > > ,) { let mut stream = BufReader :: new (stream) ; let driver_info : DriverInfo = deserialize_line (& mut stream) ; let unseen = seen . lock () . unwrap () . insert (driver_info . clone ()) ; let ignored = options . ignore . contains (& driver_info . package_name) ; let should_run = unseen && ! ignored ; serialize_line (& should_run , stream . get_mut ()) ; let mut stderr = String :: new () ; stream . read_to_string (& mut stderr) . unwrap () ; let base_url = format ! ("https://docs.rs/crate/{}/{}/source/src/{{file}}#{{line}}" , driver_info . package_name , driver_info . version) ; let messages = stderr . lines () . filter_map (| json_msg | serde_json :: from_str :: < Diagnostic > (json_msg) . ok ()) . filter_map (| diag | ClippyWarning :: new (diag , & base_url , & driver_info . package_name)) ; for message in messages { sender . send (message) . unwrap () ; } }
};
}
