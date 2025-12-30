// Generated macro for CGEventTap (struct)
macro_rules! Depcrate_eventCGEventTap {
() => {
// Module: crate::event
// Provides: {"CGEventTap"}
// Dependencies: {}
# [doc = " ```no_run"] # [doc = " use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};"] # [doc = " use core_graphics::event::{CGEventTap, CGEventTapLocation, CGEventTapPlacement, CGEventTapOptions, CGEventType, CallbackResult};"] # [doc = " let current = CFRunLoop::get_current();"] # [doc = ""] # [doc = " CGEventTap::with_enabled("] # [doc = "     CGEventTapLocation::HID,"] # [doc = "     CGEventTapPlacement::HeadInsertEventTap,"] # [doc = "     CGEventTapOptions::Default,"] # [doc = "     vec![CGEventType::MouseMoved],"] # [doc = "     |_proxy, _type, event| {"] # [doc = "         println!(\"{:?}\", event.location());"] # [doc = "         CallbackResult::Keep"] # [doc = "     },"] # [doc = "     ||  CFRunLoop::run_current(),"] # [doc = " ).expect(\"Failed to install event tap\");"] # [doc = " ```"] # [must_use = "CGEventTap is disabled when dropped"] pub struct CGEventTap < 'tap_life > { mach_port : CFMachPort , _callback : Box < CGEventTapCallbackFn < 'tap_life > > , }
};
}
