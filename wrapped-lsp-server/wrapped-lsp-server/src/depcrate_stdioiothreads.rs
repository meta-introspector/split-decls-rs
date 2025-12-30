// Generated macro for IoThreads (struct)
macro_rules! Depcrate_stdioIoThreads {
() => {
// Module: crate::stdio
// Provides: {"IoThreads"}
// Dependencies: {}
pub struct IoThreads { reader : thread :: JoinHandle < io :: Result < () > > , writer : thread :: JoinHandle < io :: Result < () > > , dropper : thread :: JoinHandle < () > , }
};
}
