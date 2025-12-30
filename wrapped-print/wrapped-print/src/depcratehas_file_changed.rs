// Generated macro for has_file_changed (function)
macro_rules! Depcratehas_file_changed {
() => {
// Module: crate
// Provides: {"has_file_changed"}
// Dependencies: {}
async fn has_file_changed (rx : & mut Receiver < Result < Event , notify :: Error > > , path : & PathBuf) -> bool { loop { if let Some (Ok (event)) = rx . recv () . await { if event . paths . contains (path) { if let notify :: EventKind :: Create (_) | notify :: EventKind :: Modify (_) = event . kind { break ; } } } } true }
};
}
