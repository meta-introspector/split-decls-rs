// Generated macro for impl_255 (impl)
macro_rules! Depcrate_appkitimpl_255 {
() => {
// Module: crate::appkit
// Provides: {"impl_255"}
// Dependencies: {}
impl NSScreen for id { unsafe fn mainScreen (_ : Self) -> id { msg_send ! [class ! (NSScreen) , mainScreen] } unsafe fn deepestScreen (_ : Self) -> id { msg_send ! [class ! (NSScreen) , deepestScreen] } unsafe fn screens (_ : Self) -> id { msg_send ! [class ! (NSScreen) , screens] } unsafe fn depth (self) -> NSWindowDepth { msg_send ! [self , depth] } unsafe fn frame (self) -> NSRect { msg_send ! [self , frame] } unsafe fn supportedWindowDepths (self) -> * const NSWindowDepth { msg_send ! [self , supportedWindowDepths] } unsafe fn deviceDescription (self) -> id { msg_send ! [self , deviceDescription] } unsafe fn visibleFrame (self) -> NSRect { msg_send ! [self , visibleFrame] } unsafe fn colorSpace (self) -> id { msg_send ! [self , colorSpace] } unsafe fn screensHaveSeparateSpaces (_ : Self) -> BOOL { msg_send ! [class ! (NSScreen) , screensHaveSeparateSpaces] } unsafe fn maximumRefreshInterval (self) -> NSTimeInterval { msg_send ! [self , maximumRefreshInterval] } unsafe fn minimumRefreshInterval (self) -> NSTimeInterval { msg_send ! [self , minimumRefreshInterval] } unsafe fn backingAlignedRect_options_ (self , aRect : NSRect , options : NSAlignmentOptions ,) -> NSRect { msg_send ! [self , backingAlignedRect : aRect options : options] } unsafe fn backingScaleFactor (self) -> CGFloat { msg_send ! [self , backingScaleFactor] } unsafe fn convertRectFromBacking_ (self , aRect : NSRect) -> NSRect { msg_send ! [self , convertRectFromBacking : aRect] } unsafe fn convertRectToBacking_ (self , aRect : NSRect) -> NSRect { msg_send ! [self , convertRectToBacking : aRect] } }
};
}
