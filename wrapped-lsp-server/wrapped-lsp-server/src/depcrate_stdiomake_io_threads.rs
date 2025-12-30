// Generated macro for make_io_threads (function)
macro_rules! Depcrate_stdiomake_io_threads {
() => {
// Module: crate::stdio
// Provides: {"make_io_threads"}
// Dependencies: {}
pub (crate) fn make_io_threads (reader : thread :: JoinHandle < io :: Result < () > > , writer : thread :: JoinHandle < io :: Result < () > > , dropper : thread :: JoinHandle < () > ,) -> IoThreads { IoThreads { reader , writer , dropper } }
};
}
