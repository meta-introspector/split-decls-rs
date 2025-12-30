// Generated macro for ReadySendRequest (struct)
macro_rules! Depcrate_clientReadySendRequest {
() => {
// Module: crate::client
// Provides: {"ReadySendRequest"}
// Dependencies: {}
# [doc = " Returns a `SendRequest` instance once it is ready to send at least one"] # [doc = " request."] # [derive (Debug)] pub struct ReadySendRequest < B : Buf > { inner : Option < SendRequest < B > > , }
};
}
