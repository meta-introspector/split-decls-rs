// Generated macro for write_inline_diff (function)
macro_rules! Depcrate_printerwrite_inline_diff {
() => {
// Module: crate::printer
// Provides: {"write_inline_diff"}
// Dependencies: {}
# [doc = " Format a single line to show an inline diff of the two strings given."] # [doc = ""] # [doc = " The given strings should not have a trailing newline."] # [doc = ""] # [doc = " The output of this function will be two lines, each with a trailing newline."] fn write_inline_diff < TWrite : fmt :: Write > (f : & mut TWrite , left : & str , right : & str) -> fmt :: Result { let diff = :: diff :: chars (left , right) ; let mut writer = InlineWriter :: new (f) ; let light = Red ; let heavy = Red . on_fixed (52) . bold () ; writer . write_with_style (& SIGN_LEFT , light) ? ; for change in diff . iter () { match change { :: diff :: Result :: Both (value , _) => writer . write_with_style (value , light) ? , :: diff :: Result :: Left (value) => writer . write_with_style (value , heavy) ? , _ => () , } } writer . finish () ? ; let light = Green ; let heavy = Green . on_fixed (22) . bold () ; writer . write_with_style (& SIGN_RIGHT , light) ? ; for change in diff . iter () { match change { :: diff :: Result :: Both (value , _) => writer . write_with_style (value , light) ? , :: diff :: Result :: Right (value) => writer . write_with_style (value , heavy) ? , _ => () , } } writer . finish () }
};
}
