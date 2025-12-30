// Generated macro for spawn_command (function)
macro_rules! Depcrate_processspawn_command {
() => {
// Module: crate::process
// Provides: {"spawn_command"}
// Dependencies: {}
fn spawn_command (command : Command , size : Option < COORD >) -> Result < Process , Error > { let _ = enableVirtualTerminalSequenceProcessing () ; let size = size . or_else (| | inhirentConsoleSize () . ok ()) . unwrap_or (COORD { X : 80 , Y : 25 }) ; let (mut console , output , input) = createPseudoConsole (size) ? ; let startup_info = initializeStartupInfoAttachedToConPTY (& mut console) ? ; let proc = execProc (command , startup_info) ? ; Ok (Process { input , output , _console : console , _proc : proc , _proc_info : startup_info , }) }
};
}
