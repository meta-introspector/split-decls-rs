// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl CommandExt for Command { fn status_without_output (& mut self) -> io :: Result < std :: process :: ExitStatus > { self . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () } fn spawn_detached (& mut self) -> io :: Result < () > { self . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) ; # [cfg (unix)] unsafe { use std :: os :: unix :: process :: CommandExt as _ ; self . pre_exec (move | | { match libc :: fork () { - 1 => return Err (io :: Error :: last_os_error ()) , 0 => () , _ => libc :: _exit (0) , } if libc :: setsid () == - 1 { return Err (io :: Error :: last_os_error ()) ; } Ok (()) }) ; } # [cfg (windows)] { use std :: os :: windows :: process :: CommandExt ; const CREATE_NEW_PROCESS_GROUP : u32 = 0x00000200 ; const CREATE_NO_WINDOW : u32 = 0x08000000 ; self . creation_flags (CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW) ; } self . spawn () . map (| _ | ()) } }
};
}
