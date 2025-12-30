// Generated macro for SetForegroundColor (struct)
macro_rules! Depcrate_styleSetForegroundColor {
() => {
// Module: crate::style
// Provides: {"SetForegroundColor"}
// Dependencies: {}
# [doc = " A command that sets the the foreground color."] # [doc = ""] # [doc = " See [`Color`](enum.Color.html) for more info."] # [doc = ""] # [doc = " [`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background"] # [doc = " color in one command."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct SetForegroundColor (pub Color) ;
};
}
