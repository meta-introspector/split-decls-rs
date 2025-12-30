// Generated macro for impl_20 (impl)
macro_rules! Depcrate_resolutionimpl_20 {
() => {
// Module: crate::resolution
// Provides: {"impl_20"}
// Dependencies: {}
impl ResolutionManager { pub fn new (resolution_dir : PathBuf) -> Self { ResolutionManager { resolution_dir } } pub fn run (& self) { println ! ("ResolutionManager started, watching: {:?}" , self . resolution_dir) ; let (tx , rx) = std :: sync :: mpsc :: channel () ; let mut watcher = RecommendedWatcher :: new (tx , notify :: Config :: default ()) . unwrap () ; watcher . watch (& self . resolution_dir , RecursiveMode :: NonRecursive) . unwrap () ; for res in rx { match res { Ok (event) => { if event . kind . is_create () || event . kind . is_modify () { for path in event . paths { println ! ("Resolution file detected: {:?}" , path) ; if let Ok (file_content) = fs :: read_to_string (& path) { if let Ok (resolution) = serde_json :: from_str :: < Resolution > (& file_content) { println ! ("Successfully deserialized resolution: {:?}" , resolution) ; self . apply_resolution (resolution) ; fs :: remove_file (& path) . expect ("Failed to remove resolution file") ; } } } } } Err (e) => println ! ("watch error: {:?}" , e) , } } } fn apply_resolution (& self , resolution : Resolution) { println ! ("Applying resolution: {:?}" , resolution) ; match resolution { Resolution :: Continue => { } Resolution :: ModifyCode { file , line , column , new_code } => { println ! ("  -> Modifying {} at {}:{} with: {}" , file , line , column , new_code) ; } } } }
};
}
