// Generated macro for impl_14 (impl)
macro_rules! Depcrate_codec_errorimpl_14 {
() => {
// Module: crate::codec::error
// Provides: {"impl_14"}
// Dependencies: {}
impl fmt :: Display for UserError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { use self :: UserError :: * ; fmt . write_str (match * self { InactiveStreamId => "inactive stream" , UnexpectedFrameType => "unexpected frame type" , PayloadTooBig => "payload too big" , Rejected => "rejected" , ReleaseCapacityTooBig => "release capacity too big" , OverflowedStreamId => "stream ID overflowed" , MalformedHeaders => "malformed headers" , MissingUriSchemeAndAuthority => "request URI missing scheme and authority" , PollResetAfterSendResponse => "poll_reset after send_response is illegal" , SendPingWhilePending => "send_ping before received previous pong" , SendSettingsWhilePending => "sending SETTINGS before received previous ACK" , PeerDisabledServerPush => "sending PUSH_PROMISE to peer who disabled server push" , }) } }
};
}
