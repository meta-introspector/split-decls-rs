// Generated macro for Process (trait)
macro_rules! Depcrate_processProcess {
() => {
// Module: crate::process
// Provides: {"Process"}
// Dependencies: {}
# [doc = " This trait represents a platform independent process which runs a program."] pub trait Process : Sized { # [doc = " A command which process can run."] type Command ; # [doc = " A representation of IO stream of communication with a programm a process is running."] type Stream ; # [doc = " Spawn parses a given string as a commandline string and spawns it on a process."] fn spawn < S > (cmd : S) -> Result < Self > where S : AsRef < str > ; # [doc = " Spawn_command runs a process with a given command."] fn spawn_command (command : Self :: Command) -> Result < Self > ; # [doc = " It opens a IO stream with a spawned process."] fn open_stream (& mut self) -> Result < Self :: Stream > ; }
};
}
