// Generated macro for ReadStatus (enum)
macro_rules! Depcrate_recv_streamReadStatus {
() => {
// Module: crate::recv_stream
// Provides: {"ReadStatus"}
// Dependencies: {}
enum ReadStatus < T > { Readable (T) , Finished (Option < T >) , Failed (Option < T > , proto :: ReadError) , }
};
}
