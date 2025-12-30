// Generated macro for WriterThread (struct)
macro_rules! Depcrate_driver_applyWriterThread {
() => {
// Module: crate::driver::apply
// Provides: {"WriterThread"}
// Dependencies: {}
# [doc = " A helper to manage writing to stdin on a separate thread to avoid deadlock."] struct WriterThread { handle : Option < std :: thread :: JoinHandle < std :: io :: Result < () > > > , }
};
}
