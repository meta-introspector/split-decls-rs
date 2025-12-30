// Generated macro for EnvChange (struct)
macro_rules! DepcrateEnvChange {
() => {
// Module: crate
// Provides: {"EnvChange"}
// Dependencies: {}
struct EnvChange < 'snap > { changed_vars : Vec < & 'snap str > , prev_working_dir : Option < PathBuf > , snap : & 'snap EnvSnapshot , _guard : std :: sync :: MutexGuard < 'snap , () > , }
};
}
