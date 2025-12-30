// Generated macro for wrap_stream_error (function)
macro_rules! Depcrate_streamwrap_stream_error {
() => {
// Module: crate::stream
// Provides: {"wrap_stream_error"}
// Dependencies: {}
# [doc (hidden)] pub fn wrap_stream_error < T , Input > (input : & Input , err : < Input :: Error as ParseError < Input :: Token , Input :: Range , Input :: Position > > :: StreamError ,) -> ParseResult < T , < Input as StreamOnce > :: Error > where Input : ? Sized + StreamOnce + Positioned , { let err = Input :: Error :: from_error (input . position () , err) ; if input . is_partial () { CommitErr (err) } else { PeekErr (err . into ()) } }
};
}
