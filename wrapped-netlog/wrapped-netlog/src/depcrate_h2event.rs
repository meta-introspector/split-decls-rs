// Generated macro for Event (enum)
macro_rules! Depcrate_h2Event {
() => {
// Module: crate::h2
// Provides: {"Event"}
// Dependencies: {}
# [derive (Debug)] pub enum Event { Http2Session (Http2SessionEvent) , Http2SessionInitialized (Http2SessionInitializedEvent) , Http2SessionSendSettings (Http2SessionSendSettingsEvent) , Http2SessionRecvSetting (Http2SessionRecvSettingEvent) , Http2SessionSendHeaders (Http2SessionSendHeadersEvent) , Http2SessionSendData (Http2SessionSendDataEvent) , Http2SessionRecvHeaders (Http2SessionRecvHeadersEvent) , Http2SessionRecvData (Http2SessionRecvDataEvent) , Http2SessionUpdateRecvWindow (Http2SessionUpdateRecvWindowEvent) , Http2SessionUpdateSendWindow (Http2SessionUpdateSendWindowEvent) , Http2SessionUpdateStreamsSendWindowSize (Http2SessionUpdateStreamsSendWindowSizeEvent ,) , Http2SessionSendWindowUpdate (Http2SessionSendWindowUpdateEvent) , Http2SessionRecvWindowUpdate (Http2SessionRecvWindowUpdateEvent) , Http2StreamUpdateSendWindow (Http2StreamUpdateSendWindowEvent) , Http2StreamUpdateRecvWindow (Http2StreamUpdateRecvWindowEvent) , Http2StreamStalledByStreamSendWindow (Http2StreamStalledByStreamSendWindowEvent ,) , Http2SessionPing (Http2SessionPingEvent) , Http2SessionSendRstStream (Http2SessionSendRstStreamEvent) , Http2SessionRecvRstStream (Http2SessionRecvRstStreamEvent) , Http2SessionRecvGoaway (Http2SessionRecvGoawayEvent) , Http2SessionClose (Http2SessionCloseEvent) , Http2SessionStalledMaxStreams (Htt2SessionStalledMaxStreamsEvent) , }
};
}
