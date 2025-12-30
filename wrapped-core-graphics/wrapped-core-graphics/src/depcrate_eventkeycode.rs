// Generated macro for KeyCode (struct)
macro_rules! Depcrate_eventKeyCode {
() => {
// Module: crate::event
// Provides: {"KeyCode"}
// Dependencies: {}
# [doc = " Constants for the virtual key codes"] # [doc = ""] # [doc = " These constants are the virtual keycodes defined originally in"] # [doc = " Inside Mac Volume V, pg. V-191. They identify physical keys on a"] # [doc = " keyboard. The struct contains the values of the `ANSIKeyCode`,"] # [doc = " `KeyCode`, `ISOKeyCode` and `JISKeyCode` of the original Carbon headers."] # [doc = ""] # [doc = " Those constants with \"ANSI\" in the name are labeled"] # [doc = " according to the key position on an ANSI-standard US keyboard."] # [doc = " For example, `ANSI_A` indicates the virtual keycode for the key"] # [doc = " with the letter 'A' in the US keyboard layout. Other keyboard"] # [doc = " layouts may have the 'A' key label on a different physical key;"] # [doc = " in this case, pressing 'A' will generate a different virtual"] # [doc = " keycode. Constants with the 'JIS_' or 'ISO_' prefix behave"] # [doc = " analogously. Keys without a prefix are independent of the"] # [doc = " keyboard layout."] # [doc = ""] # [doc = " [Ref](https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.13.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h#L197-L327)"] # [repr (C)] pub struct KeyCode ;
};
}
