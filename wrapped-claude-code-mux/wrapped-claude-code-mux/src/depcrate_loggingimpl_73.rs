// Generated macro for impl_73 (impl)
macro_rules! Depcrate_loggingimpl_73 {
() => {
// Module: crate::logging
// Provides: {"impl_73"}
// Dependencies: {}
impl QueryableLogLayer { pub fn new (buffer : Arc < RwLock < VecDeque < LogEntry > > > , log_file_path : & str ,) -> anyhow :: Result < Self > { let file = OpenOptions :: new () . create (true) . write (true) . append (true) . open (log_file_path) ? ; Ok (Self { buffer , log_file : Arc :: new (RwLock :: new (file)) , }) } }
};
}
