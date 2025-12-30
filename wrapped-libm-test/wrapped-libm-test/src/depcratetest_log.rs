// Generated macro for test_log (function)
macro_rules! Depcratetest_log {
() => {
// Module: crate
// Provides: {"test_log"}
// Dependencies: {}
# [doc = " Print to stderr and additionally log it to `target/test-log.txt`. This is useful for saving"] # [doc = " output that would otherwise be consumed by the test harness."] pub fn test_log (s : & str) { static OUTFILE : LazyLock < Option < File > > = LazyLock :: new (| | { let target_dir = match env :: var ("CARGO_TARGET_DIR") { Ok (s) => PathBuf :: from (s) , Err (_) => { let Ok (x) = env :: var ("CARGO_MANIFEST_DIR") else { return None ; } ; PathBuf :: from (x) . join ("../target") } } ; let outfile = target_dir . join ("test-log.txt") ; let mut f = File :: options () . create (true) . append (true) . open (outfile) . expect ("failed to open logfile") ; let now = SystemTime :: now () . duration_since (SystemTime :: UNIX_EPOCH) . unwrap () ; writeln ! (f , "\n\nTest run at {}" , now . as_secs ()) . unwrap () ; writeln ! (f , "arch: {}" , env :: consts :: ARCH) . unwrap () ; writeln ! (f , "os: {}" , env :: consts :: OS) . unwrap () ; writeln ! (f , "bits: {}" , usize :: BITS) . unwrap () ; writeln ! (f , "emulated: {}" , emulated ()) . unwrap () ; writeln ! (f , "ci: {}" , ci ()) . unwrap () ; writeln ! (f , "cargo features: {}" , env ! ("CFG_CARGO_FEATURES")) . unwrap () ; writeln ! (f , "opt level: {}" , env ! ("CFG_OPT_LEVEL")) . unwrap () ; writeln ! (f , "target features: {}" , env ! ("CFG_TARGET_FEATURES")) . unwrap () ; writeln ! (f , "extensive iterations {}" , extensive_max_iterations ()) . unwrap () ; Some (f) }) ; eprintln ! ("{s}") ; if let Some (mut f) = OUTFILE . as_ref () { writeln ! (f , "{s}") . unwrap () ; } }
};
}
