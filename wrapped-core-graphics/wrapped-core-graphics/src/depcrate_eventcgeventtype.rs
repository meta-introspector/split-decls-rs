// Generated macro for CGEventType (enum)
macro_rules! Depcrate_eventCGEventType {
() => {
// Module: crate::event
// Provides: {"CGEventType"}
// Dependencies: {}
# [doc = " Constants that specify the different types of input events."] # [doc = ""] # [doc = " [Ref](http://opensource.apple.com/source/IOHIDFamily/IOHIDFamily-700/IOHIDSystem/IOKit/hidsystem/IOLLEvent.h)"] # [repr (u32)] # [derive (Clone , Copy , Debug)] pub enum CGEventType { Null = 0 , LeftMouseDown = 1 , LeftMouseUp = 2 , RightMouseDown = 3 , RightMouseUp = 4 , MouseMoved = 5 , LeftMouseDragged = 6 , RightMouseDragged = 7 , KeyDown = 10 , KeyUp = 11 , FlagsChanged = 12 , ScrollWheel = 22 , TabletPointer = 23 , TabletProximity = 24 , OtherMouseDown = 25 , OtherMouseUp = 26 , OtherMouseDragged = 27 , TapDisabledByTimeout = 0xFFFFFFFE , TapDisabledByUserInput = 0xFFFFFFFF , }
};
}
