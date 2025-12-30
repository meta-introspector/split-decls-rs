// Generated macro for EnableBlinking (struct)
macro_rules! Depcrate_cursorEnableBlinking {
() => {
// Module: crate::cursor
// Provides: {"EnableBlinking"}
// Dependencies: {}
# [doc = " A command that enables blinking of the terminal cursor."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " - Some Unix terminals (ex: GNOME and Konsole) as well as Windows versions lower than Windows 10 do not support this functionality."] # [doc = "   Use `SetCursorStyle` for better cross-compatibility."] # [doc = " - Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct EnableBlinking ;
};
}
