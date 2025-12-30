// Generated macro for platform (module)
macro_rules! Depcrate_streamplatform {
() => {
// Module: crate::stream
// Provides: {"platform"}
// Dependencies: {}
# [cfg (windows)] mod platform { impl super :: WinconStream for std :: io :: StdoutLock < '_ > { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { let initial = crate :: windows :: stdout_initial_colors () ; crate :: windows :: write_colored (self , fg , bg , data , initial) } } impl super :: WinconStream for std :: io :: StderrLock < '_ > { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { let initial = crate :: windows :: stderr_initial_colors () ; crate :: windows :: write_colored (self , fg , bg , data , initial) } } }
};
}
