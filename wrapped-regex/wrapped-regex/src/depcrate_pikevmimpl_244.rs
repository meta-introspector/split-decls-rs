// Generated macro for impl_244 (impl)
macro_rules! Depcrate_pikevmimpl_244 {
() => {
// Module: crate::pikevm
// Provides: {"impl_244"}
// Dependencies: {}
impl Cache { # [doc = " Create a new allocation used by the NFA machine to record execution"] # [doc = " and captures."] pub fn new (_prog : & Program) -> Self { Cache { clist : Threads :: new () , nlist : Threads :: new () , stack : vec ! [] , } } }
};
}
