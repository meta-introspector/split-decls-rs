// Generated macro for CommandHandle (struct)
macro_rules! Depcrate_commandCommandHandle {
() => {
// Module: crate::command
// Provides: {"CommandHandle"}
// Dependencies: {}
# [doc = " A handle to a cargo process used for fly-checking."] pub (crate) struct CommandHandle < T > { # [doc = " The handle to the actual cargo process. As we cannot cancel directly from with"] # [doc = " a read syscall dropping and therefore terminating the process is our best option."] child : JodGroupChild , thread : stdx :: thread :: JoinHandle < io :: Result < (bool , String) > > , program : OsString , arguments : Vec < OsString > , current_dir : Option < PathBuf > , _phantom : PhantomData < T > , }
};
}
