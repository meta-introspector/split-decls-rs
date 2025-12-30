// Generated macro for JoinHandle (struct)
macro_rules! Depcrate_threadJoinHandle {
() => {
// Module: crate::thread
// Provides: {"JoinHandle"}
// Dependencies: {}
# [doc = " Mock implementation of `std::thread::JoinHandle`."] pub struct JoinHandle < T > { result : Arc < Mutex < Option < std :: thread :: Result < T > > > > , notify : rt :: Notify , thread : Thread , }
};
}
