// Generated macro for wide (function)
macro_rules! Depcrate_windowswide {
() => {
// Module: crate::windows
// Provides: {"wide"}
// Dependencies: {}
# [doc = " Encodes as wide and adds a null character."] # [cfg (feature = "shellexecute-on-windows")] # [inline] fn wide < T : AsRef < OsStr > > (input : T) -> Vec < u16 > { use std :: os :: windows :: ffi :: OsStrExt ; input . as_ref () . encode_wide () . chain (std :: iter :: once (0)) . collect () }
};
}
