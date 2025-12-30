// Generated macro for IoWriter (struct)
macro_rules! DepcrateIoWriter {
() => {
// Module: crate
// Provides: {"IoWriter"}
// Dependencies: {}
# [doc = " This wrapper exists because we can't have both a blanket implementation"] # [doc = " for all types implementing `Write` and types of the for `&mut W` where"] # [doc = " `W: StrWrite`. Since we need the latter a lot, we choose to wrap"] # [doc = " `Write` types."] # [derive (Debug)] # [cfg (feature = "std")] pub struct IoWriter < W > (pub W) ;
};
}
