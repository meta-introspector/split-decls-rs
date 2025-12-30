// Generated macro for NSApplication (trait)
macro_rules! Depcrate_appkitNSApplication {
() => {
// Module: crate::appkit
// Provides: {"NSApplication"}
// Dependencies: {}
pub trait NSApplication : Sized { unsafe fn sharedApplication (_ : Self) -> id { msg_send ! [class ! (NSApplication) , sharedApplication] } unsafe fn mainMenu (self) -> id ; unsafe fn setActivationPolicy_ (self , policy : NSApplicationActivationPolicy) -> BOOL ; unsafe fn setPresentationOptions_ (self , options : NSApplicationPresentationOptions) -> BOOL ; unsafe fn presentationOptions_ (self) -> NSApplicationPresentationOptions ; unsafe fn setMainMenu_ (self , menu : id) ; unsafe fn setServicesMenu_ (self , menu : id) ; unsafe fn setWindowsMenu_ (self , menu : id) ; unsafe fn activateIgnoringOtherApps_ (self , ignore : BOOL) ; unsafe fn run (self) ; unsafe fn finishLaunching (self) ; unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_ (self , mask : NSUInteger , expiration : id , in_mode : id , dequeue : BOOL ,) -> id ; unsafe fn sendEvent_ (self , an_event : id) ; unsafe fn postEvent_atStart_ (self , anEvent : id , flag : BOOL) ; unsafe fn stop_ (self , sender : id) ; unsafe fn setApplicationIconImage_ (self , image : id) ; unsafe fn requestUserAttention_ (self , requestType : NSRequestUserAttentionType) ; }
};
}
