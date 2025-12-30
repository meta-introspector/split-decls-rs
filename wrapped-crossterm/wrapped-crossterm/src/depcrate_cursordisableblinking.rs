// Generated macro for DisableBlinking (struct)
macro_rules! Depcrate_cursorDisableBlinking {
() => {
// Module: crate::cursor
// Provides: {"DisableBlinking"}
// Dependencies: {}
# [doc = " A command that disables blinking of the terminal cursor."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " - Some Unix terminals (ex: GNOME and Konsole) as well as Windows versions lower than Windows 10 do not support this functionality."] # [doc = "   Use `SetCursorStyle` for better cross-compatibility."] # [doc = " - Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DisableBlinking ;
};
}
