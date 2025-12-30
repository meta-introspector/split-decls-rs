// Generated macro for impl_13 (impl)
macro_rules! Depcrate_continuationimpl_13 {
() => {
// Module: crate::continuation
// Provides: {"impl_13"}
// Dependencies: {}
impl Continue for DefaultContinuation { fn continue_execution (& self , state : CapturedState) -> Resolution { let id = & state . session_id ; let state_file = self . state_file_path (& id) ; let state_json = serde_json :: to_string_pretty (& state) . expect ("Failed to serialize captured state") ; fs :: write (& state_file , state_json) . expect (& format ! ("Failed to write state to {:?}" , state_file)) ; println ! ("State captured to: {:?}" , state_file) ; let resolution_file = self . resolution_file_path (& id) ; println ! ("Waiting for resolution file: {:?}" , resolution_file) ; loop { if resolution_file . exists () { let resolution_json = fs :: read_to_string (& resolution_file) . expect (& format ! ("Failed to read resolution from {:?}" , resolution_file)) ; let resolution : Resolution = serde_json :: from_str (& resolution_json) . expect ("Failed to deserialize resolution") ; println ! ("Resolution received: {:?}" , resolution) ; fs :: remove_file (& resolution_file) . expect ("Failed to remove resolution file") ; return resolution ; } thread :: sleep (Duration :: from_millis (500)) ; } } fn initialize (& self) -> Result < () , Box < dyn std :: error :: Error > > { fs :: create_dir_all (& self . state_dir) ? ; fs :: create_dir_all (& self . resolution_dir) ? ; Ok (()) } }
};
}
