// Generated macro for block_on (function)
macro_rules! Depcrate_reactorblock_on {
() => {
// Module: crate::reactor
// Provides: {"block_on"}
// Dependencies: {}
pub (crate) fn block_on < F : std :: future :: Future < Output = T > , T > (future : F) -> T { # [cfg (feature = "async-io")] let run = | | async_io :: block_on (future) ; # [cfg (not (feature = "async-io"))] let run = | | futures_lite :: future :: block_on (future) ; # [cfg (feature = "tokio")] let _tokio_enter = crate :: tokio :: enter () ; run () }
};
}
