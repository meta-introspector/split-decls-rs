// Generated macro for stdin_stdout_to_console (function)
macro_rules! Depcrate_stdiostdin_stdout_to_console {
() => {
// Module: crate::stdio
// Provides: {"stdin_stdout_to_console"}
// Dependencies: {}
# [doc = " Reset stdin and stdout to the attached console / tty for the duration of the closure."] # [doc = " If no console is available, stdin and stdout will be redirected to null."] pub fn stdin_stdout_to_console < F , T > (f : F) -> Result < T , Error > where F : FnOnce () -> T , { let open_write = | f | std :: fs :: OpenOptions :: new () . write (true) . open (f) ; let mut stdin = File :: open (imp :: IN_DEVICE) . or_else (| _ | File :: open (imp :: NULL_DEVICE)) ? ; let mut stdout = open_write (imp :: OUT_DEVICE) . or_else (| _ | open_write (imp :: NULL_DEVICE)) ? ; let _stdin_guard = imp :: ReplacementGuard :: new (Stdio :: Stdin , & mut stdin) ? ; let _stdout_guard = imp :: ReplacementGuard :: new (Stdio :: Stdout , & mut stdout) ? ; Ok (f ()) }
};
}
