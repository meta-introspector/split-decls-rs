// Generated macro for FmtWriter (struct)
macro_rules! DepcrateFmtWriter {
() => {
// Module: crate
// Provides: {"FmtWriter"}
// Dependencies: {}
# [doc = " This wrapper exists because we can't have both a blanket implementation"] # [doc = " for all types implementing `io::Write` and types of the form `&mut W` where"] # [doc = " `W: StrWrite`. Since we need the latter a lot, we choose to wrap"] # [doc = " `Write` types."] # [derive (Debug)] pub struct FmtWriter < W > (pub W) ;
};
}
