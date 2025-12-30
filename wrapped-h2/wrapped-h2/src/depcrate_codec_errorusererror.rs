// Generated macro for UserError (enum)
macro_rules! Depcrate_codec_errorUserError {
() => {
// Module: crate::codec::error
// Provides: {"UserError"}
// Dependencies: {}
# [doc = " Errors caused by users of the library"] # [derive (Debug)] pub enum UserError { # [doc = " The stream ID is no longer accepting frames."] InactiveStreamId , # [doc = " The stream is not currently expecting a frame of this type."] UnexpectedFrameType , # [doc = " The payload size is too big"] PayloadTooBig , # [doc = " The application attempted to initiate too many streams to remote."] Rejected , # [doc = " The released capacity is larger than claimed capacity."] ReleaseCapacityTooBig , # [doc = " The stream ID space is overflowed."] # [doc = ""] # [doc = " A new connection is needed."] OverflowedStreamId , # [doc = " Illegal headers, such as connection-specific headers."] MalformedHeaders , # [doc = " Request submitted with relative URI."] MissingUriSchemeAndAuthority , # [doc = " Calls `SendResponse::poll_reset` after having called `send_response`."] PollResetAfterSendResponse , # [doc = " Calls `PingPong::send_ping` before receiving a pong."] SendPingWhilePending , # [doc = " Tries to update local SETTINGS while ACK has not been received."] SendSettingsWhilePending , # [doc = " Tries to send push promise to peer who has disabled server push"] PeerDisabledServerPush , }
};
}
