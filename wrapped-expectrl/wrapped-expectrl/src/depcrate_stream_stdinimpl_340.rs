// Generated macro for impl_340 (impl)
macro_rules! Depcrate_stream_stdinimpl_340 {
() => {
// Module: crate::stream::stdin
// Provides: {"impl_340"}
// Dependencies: {}
impl Stdin { # [doc = " Creates a new instance of Stdin."] # [doc = ""] # [doc = " It may change terminal's STDIN state therefore, after"] # [doc = " it's used you must call [Stdin::close]."] pub fn open () -> Result < Self , Error > { # [cfg (not (feature = "async"))] { let mut stdin = inner :: StdinInner :: new () . map (| inner | Self { inner }) ? ; stdin . blocking (true) ? ; Ok (stdin) } # [cfg (feature = "async")] { let stdin = inner :: StdinInner :: new () . map (| inner | Self { inner }) ? ; Ok (stdin) } } # [doc = " Close frees a resources which were used."] # [doc = ""] # [doc = " It must be called [Stdin] was used."] # [doc = " Otherwise the STDIN might be returned to original state."] pub fn close (mut self) -> Result < () , Error > { # [cfg (not (feature = "async"))] self . blocking (false) ? ; self . inner . close () ? ; Ok (()) } # [cfg (not (feature = "async"))] pub (crate) fn blocking (& mut self , on : bool) -> Result < () , Error > { self . inner . blocking (on) } }
};
}
