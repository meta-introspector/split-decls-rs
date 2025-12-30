// Generated macro for impl_270 (impl)
macro_rules! Depcrate_appkitimpl_270 {
() => {
// Module: crate::appkit
// Provides: {"impl_270"}
// Dependencies: {}
impl NSSound for id { unsafe fn initWithContentsOfFile_withReference_ (self , filepath : id , byRef : BOOL) -> id { msg_send ! [self , initWithContentsOfFile : filepath withReference : byRef] } unsafe fn initWithContentsOfURL_withReference_ (self , fileUrl : id , byRef : BOOL) -> id { msg_send ! [self , initWithContentsOfURL : fileUrl withReference : byRef] } unsafe fn initWithData_ (self , audioData : id) -> id { msg_send ! [self , initWithData : audioData] } unsafe fn initWithPasteboard_ (self , pasteboard : id) -> id { msg_send ! [self , initWithPasteboard : pasteboard] } unsafe fn name (self) -> id { msg_send ! [self , name] } unsafe fn volume (self) -> f32 { msg_send ! [self , volume] } unsafe fn currentTime (self) -> NSTimeInterval { msg_send ! [self , currentTime] } unsafe fn loops (self) -> BOOL { msg_send ! [self , loops] } unsafe fn playbackDeviceIdentifier (self) -> id { msg_send ! [self , playbackDeviceIdentifier] } unsafe fn delegate (self) -> id { msg_send ! [self , delegate] } unsafe fn duration (self) -> NSTimeInterval { msg_send ! [self , duration] } unsafe fn playing (self) -> BOOL { msg_send ! [self , playing] } unsafe fn pause (self) -> BOOL { msg_send ! [self , pause] } unsafe fn play (self) -> BOOL { msg_send ! [self , play] } unsafe fn resume (self) -> BOOL { msg_send ! [self , resume] } unsafe fn stop (self) -> BOOL { msg_send ! [self , stop] } unsafe fn writeToPasteboard_ (self , pasteboard : id) { msg_send ! [self , writeToPasteboard : pasteboard] } }
};
}
