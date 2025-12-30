// Generated macro for EnableBracketedPaste (struct)
macro_rules! Depcrate_eventEnableBracketedPaste {
() => {
// Module: crate::event
// Provides: {"EnableBracketedPaste"}
// Dependencies: {}
# [doc = " A command that enables [bracketed paste mode](https://en.wikipedia.org/wiki/Bracketed-paste)."] # [doc = ""] # [doc = " It should be paired with [`DisableBracketedPaste`] at the end of execution."] # [doc = ""] # [doc = " This is not supported in older Windows terminals without"] # [doc = " [virtual terminal sequences](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences)."] # [cfg (feature = "bracketed-paste")] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct EnableBracketedPaste ;
};
}
