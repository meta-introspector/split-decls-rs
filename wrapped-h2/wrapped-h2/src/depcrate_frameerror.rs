// Generated macro for Error (enum)
macro_rules! Depcrate_frameError {
() => {
// Module: crate::frame
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors that can occur during parsing an HTTP/2 frame."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum Error { # [doc = " A length value other than 8 was set on a PING message."] BadFrameSize , # [doc = " The padding length was larger than the frame-header-specified"] # [doc = " length of the payload."] TooMuchPadding , # [doc = " An invalid setting value was provided"] InvalidSettingValue , # [doc = " An invalid window update value"] InvalidWindowUpdateValue , # [doc = " The payload length specified by the frame header was not the"] # [doc = " value necessary for the specific frame type."] InvalidPayloadLength , # [doc = " Received a payload with an ACK settings frame"] InvalidPayloadAckSettings , # [doc = " An invalid stream identifier was provided."] # [doc = ""] # [doc = " This is returned if a SETTINGS or PING frame is received with a stream"] # [doc = " identifier other than zero."] InvalidStreamId , # [doc = " A request or response is malformed."] MalformedMessage , # [doc = " An invalid stream dependency ID was provided"] # [doc = ""] # [doc = " This is returned if a HEADERS or PRIORITY frame is received with an"] # [doc = " invalid stream identifier."] InvalidDependencyId , # [doc = " Failed to perform HPACK decoding"] Hpack (hpack :: DecoderError) , }
};
}
