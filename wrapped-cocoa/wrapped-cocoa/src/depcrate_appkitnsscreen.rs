// Generated macro for NSScreen (trait)
macro_rules! Depcrate_appkitNSScreen {
() => {
// Module: crate::appkit
// Provides: {"NSScreen"}
// Dependencies: {}
pub trait NSScreen : Sized { unsafe fn mainScreen (_ : Self) -> id ; unsafe fn deepestScreen (_ : Self) -> id ; unsafe fn screens (_ : Self) -> id ; unsafe fn depth (self) -> NSWindowDepth ; unsafe fn frame (self) -> NSRect ; unsafe fn supportedWindowDepths (self) -> * const NSWindowDepth ; unsafe fn deviceDescription (self) -> id ; unsafe fn visibleFrame (self) -> NSRect ; unsafe fn colorSpace (self) -> id ; unsafe fn screensHaveSeparateSpaces (_ : Self) -> BOOL ; unsafe fn maximumRefreshInterval (self) -> NSTimeInterval ; unsafe fn minimumRefreshInterval (self) -> NSTimeInterval ; unsafe fn backingAlignedRect_options_ (self , aRect : NSRect , options : NSAlignmentOptions ,) -> NSRect ; unsafe fn backingScaleFactor (self) -> CGFloat ; unsafe fn convertRectFromBacking_ (self , aRect : NSRect) -> NSRect ; unsafe fn convertRectToBacking_ (self , aRect : NSRect) -> NSRect ; }
};
}
