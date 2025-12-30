// Generated macro for NSSound (trait)
macro_rules! Depcrate_appkitNSSound {
() => {
// Module: crate::appkit
// Provides: {"NSSound"}
// Dependencies: {}
pub trait NSSound : Sized { unsafe fn canInitWithPasteboard_ (_ : Self , pasteboard : id) -> BOOL { msg_send ! [class ! (NSSound) , canInitWithPasteboard : pasteboard] } unsafe fn initWithContentsOfFile_withReference_ (self , filepath : id , byRef : BOOL) -> id ; unsafe fn initWithContentsOfURL_withReference_ (self , fileUrl : id , byRef : BOOL) -> id ; unsafe fn initWithData_ (self , audioData : id) -> id ; unsafe fn initWithPasteboard_ (self , pasteboard : id) -> id ; unsafe fn name (self) -> id ; unsafe fn volume (self) -> f32 ; unsafe fn currentTime (self) -> NSTimeInterval ; unsafe fn loops (self) -> BOOL ; unsafe fn playbackDeviceIdentifier (self) -> id ; unsafe fn delegate (self) -> id ; unsafe fn soundUnfilteredTypes (_ : Self) -> id { msg_send ! [class ! (NSSound) , soundUnfilteredTypes] } unsafe fn soundNamed_ (_ : Self , soundName : id) -> id { msg_send ! [class ! (NSSound) , soundNamed : soundName] } unsafe fn duration (self) -> NSTimeInterval ; unsafe fn playing (self) -> BOOL ; unsafe fn pause (self) -> BOOL ; unsafe fn play (self) -> BOOL ; unsafe fn resume (self) -> BOOL ; unsafe fn stop (self) -> BOOL ; unsafe fn writeToPasteboard_ (self , pasteboard : id) ; }
};
}
