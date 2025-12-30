// Generated macro for init (function)
macro_rules! Depcrateinit {
() => {
// Module: crate
// Provides: {"init"}
// Dependencies: {}
# [ctor] fn init () { if let Ok (path) = env :: var ("RANDOMIZE_READDIR_LOG") { let path = format ! ("{}.{}" , path , process :: id ()) ; WriteLogger :: init (LevelFilter :: Info , Config :: default () , File :: create (path) . expect ("failed to create log file") ,) . expect ("failed to initialize logger") ; } let opendir = load_next :: < Opendir > (b"opendir\0") ; let fdopendir = load_next :: < Fdopendir > (b"fdopendir\0") ; let readdir = load_next :: < Readdir > (b"readdir\0") ; let readdir64 = load_next :: < Readdir64 > (b"readdir64\0") ; let closedir = load_next :: < Closedir > (b"closedir\0") ; _ = STATE . get_or_init (| | State { opendir , fdopendir , readdir , readdir64 , closedir , dirs : RwLock :: new (HashMap :: new ()) , }) ; }
};
}
