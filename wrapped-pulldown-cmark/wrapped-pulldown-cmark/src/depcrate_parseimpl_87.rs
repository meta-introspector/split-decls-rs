// Generated macro for impl_87 (impl)
macro_rules! Depcrate_parseimpl_87 {
() => {
// Module: crate::parse
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'input > Parser < 'input , DefaultBrokenLinkCallback > { # [doc = " Creates a new event iterator for a markdown string without any options enabled."] pub fn new (text : & 'input str) -> Self { Self :: new_ext (text , Options :: empty ()) } # [doc = " Creates a new event iterator for a markdown string with given options."] pub fn new_ext (text : & 'input str , options : Options) -> Self { Self :: new_with_broken_link_callback (text , options , None) } }
};
}
