// Generated macro for render_invisible (function)
macro_rules! Depcrate_outputrender_invisible {
() => {
// Module: crate::output
// Provides: {"render_invisible"}
// Dependencies: {}
fn render_invisible (s : & str , newlines_matter : bool) -> Cow < '_ , str > { if newlines_matter || s . find (& ['\x1b' , '\x07' , '\x08' , '\x7f'] [..]) . is_some () { Cow :: Owned (s . replace ('\r' , "␍\r") . replace ('\n' , "␊\n") . replace ("␍\r␊\n" , "␍␊\r\n") . replace ('\x07' , "␇") . replace ('\x08' , "␈") . replace ('\x1b' , "␛") . replace ('\x7f' , "␡") ,) } else { Cow :: Borrowed (s) } }
};
}
