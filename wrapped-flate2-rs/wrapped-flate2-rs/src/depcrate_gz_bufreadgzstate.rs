// Generated macro for GzState (enum)
macro_rules! Depcrate_gz_bufreadGzState {
() => {
// Module: crate::gz::bufread
// Provides: {"GzState"}
// Dependencies: {}
# [derive (Debug)] enum GzState { Header (GzHeaderParser) , Body (GzHeader) , Finished (GzHeader , usize , [u8 ; 8]) , Err (io :: Error) , End (Option < GzHeader >) , }
};
}
