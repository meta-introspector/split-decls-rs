// Generated macro for load_config (function)
macro_rules! Depcrate_messagesload_config {
() => {
// Module: crate::messages
// Provides: {"load_config"}
// Dependencies: {}
pub fn load_config (path : & str) -> Result < Config , Box < dyn std :: error :: Error > > { let file = std :: fs :: File :: open (path) ? ; let file_size = file . metadata () ? . len () ; if file_size == 0 { return Err ("The config file is empty." . into ()) ; } let reader = std :: io :: BufReader :: new (file) ; let config : Config = serde_json :: from_reader (reader) ? ; Ok (config) }
};
}
