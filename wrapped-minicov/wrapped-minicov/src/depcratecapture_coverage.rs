// Generated macro for capture_coverage (function)
macro_rules! Depcratecapture_coverage {
() => {
// Module: crate
// Provides: {"capture_coverage"}
// Dependencies: {}
# [doc = " Captures the coverage data for the current program and writes it into the"] # [doc = " given sink."] # [doc = ""] # [doc = " The data should be saved to a file with the `.profraw` extension, which can"] # [doc = " then be processed using the `llvm-profdata` and `llvm-cov` tools."] # [doc = ""] # [doc = " You should call `reset_coverage` afterwards if you intend to continue"] # [doc = " running the program so that future coverage can be merged with the returned"] # [doc = " captured coverage."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is not thread-safe and should not be concurrently called from"] # [doc = " multiple threads."] pub unsafe fn capture_coverage < Writer : CoverageWriter > (writer : & mut Writer ,) -> Result < () , CoverageWriteError > { check_version () ; let mut prof_writer = ProfDataWriter { Write : write_callback :: < Writer > , WriterCtx : writer as * mut Writer as * mut u8 , } ; let res = lprofWriteData (& mut prof_writer , lprofGetVPDataReader () , 0) ; if res == 0 { Ok (()) } else { Err (CoverageWriteError) } }
};
}
